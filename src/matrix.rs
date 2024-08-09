use anyhow::{anyhow, Result};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Deref, Mul};
use std::sync::mpsc;
use std::{fmt, thread};

const NUMBER_THREADS: usize = 4;

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

struct MsgOutput<T> {
    idx: usize,
    value: T,
}

struct Msg<T> {
    input: MsgInput<T>,
    result_sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

impl<T> MsgOutput<T> {
    pub fn new(idx: usize, value: T) -> Self {
        Self { idx, value }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, result_sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self {
            input,
            result_sender,
        }
    }
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Matrix {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T> Debug for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Matrix(row={}, col={}, {})", self.row, self.col, self)
    }
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    // display a matrix with shape 2x3 as {1 2 3, 4 5 6}, shape 3x2 as {1 2, 3 4, 5 6}
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }

            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[allow(dead_code)]
fn matrix_multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + AddAssign + Default + Send + 'static,
{
    if a.col != b.row {
        Err(anyhow!("Can't do multiply between matrix a and b"))
    } else {
        let senders = (0..NUMBER_THREADS)
            .map(|_| {
                let (tx, rx) = mpsc::channel::<Msg<T>>();
                thread::spawn(move || {
                    for msg in rx {
                        let value = dot_product(msg.input.row, msg.input.col)?;
                        if let Err(e) = msg.result_sender.send(MsgOutput::new(msg.input.idx, value))
                        {
                            return Err(anyhow!(format!("error:{}", e)));
                        }
                    }
                    Ok::<_, anyhow::Error>(())
                });
                tx
            })
            .collect::<Vec<_>>();

        let mut data = vec![T::default(); a.row * b.col];
        let mut receivers = Vec::with_capacity(a.row * b.col);
        for i in 0..a.row {
            for j in 0..b.col {
                let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
                let col_data = b.data[j..]
                    .iter()
                    .step_by(b.col)
                    .copied()
                    .collect::<Vec<_>>();
                let col = Vector::new(col_data);
                let idx = i * b.col + j;
                let input_msg = MsgInput::new(idx, row, col);
                let (tx, rx) = oneshot::channel();
                let msg = Msg::new(input_msg, tx);
                if let Err(e) = senders[idx % NUMBER_THREADS].send(msg) {
                    return Err(anyhow!(format!("error:{}", e)));
                }
                receivers.push(rx);
            }
        }

        for msg in receivers {
            let output = msg.recv()?;
            data[output.idx] = output.value;
        }

        let result = Matrix::new(data, a.row, b.col);
        Ok(result)
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + AddAssign + Default + Send + 'static,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        matrix_multiply(&self, &rhs).expect("Matrix multiply error")
    }
}

struct Vector<T> {
    vec: Vec<T>,
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<T> Vector<T> {
    pub fn new(vec: impl Into<Vec<T>>) -> Self {
        Self { vec: vec.into() }
    }
}

#[allow(dead_code)]
fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + AddAssign + Mul<Output = T>,
{
    if a.len() != b.len() {
        return Err(anyhow!("a.len not equals to b.len"));
    }

    let mut result = T::default();

    for (index, _) in a.iter().enumerate() {
        result += a[index] * b[index];
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = a * b;
        assert_eq!(c.col, 2);
        assert_eq!(c.row, 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        assert_eq!(format!("{:?}", c), "Matrix(row=2, col=2, {22 28, 49 64})");

        Ok(())
    }

    #[test]
    fn test_matrix_display() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4], 2, 2);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);
        let c = a * b;
        assert_eq!(c.data, vec![7, 10, 15, 22]);
        assert_eq!(format!("{}", c), "{7 10, 15 22}");
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_two_matrix_can_not_multiply() {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);
        let _ = a * b;
    }

    #[test]
    fn test_two_matrix_can_not_multiply_is_err() {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);
        let c = matrix_multiply(&a, &b);
        assert!(c.is_err());
    }
}
