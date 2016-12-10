use ndarray::Axis;
use std::collections::BTreeMap;
use join::*;
use error::*;
use types::*;
use std::string::ToString;
use std::iter::{Iterator, IntoIterator, Chain, Map, FilterMap};
use ndarray::AxisIter;
use itertools::PutBack;
use std::slice::Iter;
use std::marker::Sized;
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct DataFrame {
    pub columns: Vec<OuterType>,
    pub data: Matrix<InnerType>,
    pub index: Vec<OuterType>,
}

#[derive(Clone)]
pub struct DataFrameIterator<'a> {
    pub names: Iter<'a, OuterType>,
    pub data: AxisIter<'a, InnerType, usize>,
}

pub struct Select<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{
    data: I,
    ind: Vec<OuterType>,
}

pub struct Remove<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{
    data: I,
    ind: Vec<OuterType>,
}

pub struct Append<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{
    pub new_data: PutBack<I>,
}



pub fn join<'a, I, J>(this: I, other: J) -> InnerJoin<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>,
          J: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{
    // let this_index: BTreeMap<OuterType, usize> =
    //     this.clone().names.enumerate().map(|(x, y)| (y.clone(), x)).collect();
    // let other_index: BTreeMap<OuterType, usize> =
    //     other.clone().names.enumerate().map(|(x, y)| (y.clone(), x)).collect();

    InnerJoin::new(this, other)

}

impl<'a, I> Iterator for Select<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{
    type Item = (OuterType, RowView<'a, InnerType>);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.data.next() {

                Some((val, dat)) => {
                    if self.ind.contains(&val) {
                        return Some((val.clone(), dat));
                    } else {
                        continue;
                    }
                }
                None => return None,
            }


        }
    }
}

impl<'a, I> Iterator for Remove<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{
    type Item = (OuterType, RowView<'a, InnerType>);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.data.next() {

                Some((val, dat)) => {
                    if !self.ind.contains(&val) {

                        return Some((val.clone(), dat));
                    } else {
                        continue;
                    }
                }
                None => return None,
            }


        }
    }
}

impl<'a, I> Iterator for Append<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{
    type Item = (OuterType, RowView<'a, InnerType>);
    fn next(&mut self) -> Option<Self::Item> {
        self.new_data.next()
    }
}


pub fn select<'a, I>(df: I, ind: Vec<OuterType>) -> Select<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{

    Select {
        data: df,
        ind: ind,
    }
}

pub fn remove<'a, I>(df: I, ind: Vec<OuterType>) -> Remove<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{

    Remove {
        data: df,
        ind: ind,
    }
}



pub fn append<'a, I>(df: I, name: OuterType, data: RowView<'a, InnerType>) -> Append<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{
    let name = OuterType::from(name);
    let mut it = PutBack::new(df);
    it.put_back((name, data));
    Append { new_data: it }
}

// pub fn join<'a, I>(this: DataFrameIterator<'a>,
//                    other: &DataFrameIterator<'a>)
//                    -> Chain<Select<'a, DataFrameIterator<'a>>, Select<'a, DataFrameIterator<'a>>>
//     where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
// {
//     let this_index: BTreeMap<OuterType, usize> =
//         this.clone().names.enumerate().map(|(x, y)| (y.clone(), x)).collect();
//     let other_index: BTreeMap<OuterType, usize> =
//         other.clone().names.enumerate().map(|(x, y)| (y.clone(), x)).collect();
//     let  =
//         InnerJoin::new(this, other);
//     let i1: Vec<OuterType> =
//         idxs.iter().filter(|x| x.2.is_some()).map(|&(ref x, _, _)| x.to_owned()).collect();
//     this.select(i1.clone()).chain(other.clone().select(i1.clone()))
// }
//


impl<'a> Iterator for DataFrameIterator<'a> {
    type Item = (OuterType, RowView<'a, InnerType>);
    fn next(&mut self) -> Option<Self::Item> {
        match self.names.next() {
            Some(val) => {
                match self.data.next() {
                    Some(dat) => Some((val.clone(), dat)),
                    None => None,
                }
            }
            None => None,
        }
    }
}


pub trait DFIter<'a> {
    type DFItem;

    fn select(self, names: Vec<OuterType>) -> Select<'a, Self>
        where Self: Sized + Iterator<Item = (OuterType, RowView<'a, InnerType>)>;
    fn remove(self, names: Vec<OuterType>) -> Remove<'a, Self>
        where Self: Sized + Iterator<Item = (OuterType, RowView<'a, InnerType>)>;
    fn append(self, name: OuterType, data: RowView<'a, InnerType>) -> Append<'a, Self>
        where Self: Sized + Iterator<Item = (OuterType, RowView<'a, InnerType>)>;
    fn concat<I>(self, other: I) -> Chain<Self, I>
        where Self: Sized + Iterator<Item = (OuterType, RowView<'a, InnerType>)>,
              I: Sized + Iterator<Item = (OuterType, RowView<'a, InnerType>)>;
    fn mapdf<B, F>(self, f: F) -> Map<Self, F>
        where F: FnMut(Self::DFItem) -> B,
              Self: Sized + Iterator<Item = (OuterType, RowView<'a, InnerType>)>;
}

impl<'a> DFIter<'a> for DataFrameIterator<'a> {
    type DFItem = (OuterType, RowView<'a, InnerType>);

    fn select(self, names: Vec<OuterType>) -> Select<'a, Self> {

        select(self, names)
    }


    fn remove(self, names: Vec<OuterType>) -> Remove<'a, Self> {

        remove(self, names)

    }

    fn append(self, name: OuterType, data: RowView<'a, InnerType>) -> Append<'a, Self> {
        append(self, name, data)

    }

    fn concat<I>(self, other: I) -> Chain<Self, I>
        where I: Sized + Iterator<Item = (OuterType, RowView<'a, InnerType>)>
    {
        self.chain(other)
    }

    fn mapdf<B, F>(self, f: F) -> Map<Self, F>
        where F: FnMut(Self::DFItem) -> B
    {

        self.map(f)
    }
}

impl<'a, I> DFIter<'a> for Select<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{
    type DFItem = (OuterType, RowView<'a, InnerType>);
    fn select(self, names: Vec<OuterType>) -> Select<'a, Self> {

        select(self, names)
    }


    fn remove(self, names: Vec<OuterType>) -> Remove<'a, Self> {

        remove(self, names)

    }

    fn append(self, name: OuterType, data: RowView<'a, InnerType>) -> Append<'a, Self> {
        append(self, name, data)

    }
    fn concat<J>(self, other: J) -> Chain<Self, J>
        where J: Sized + Iterator<Item = (OuterType, RowView<'a, InnerType>)>
    {
        self.chain(other)
    }
    fn mapdf<B, F>(self, f: F) -> Map<Self, F>
        where F: FnMut(Self::DFItem) -> B
    {

        self.map(f)
    }
}

impl<'a, I> DFIter<'a> for Remove<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{
    type DFItem = (OuterType, RowView<'a, InnerType>);
    fn select(self, names: Vec<OuterType>) -> Select<'a, Self> {

        select(self, names)
    }


    fn remove(self, names: Vec<OuterType>) -> Remove<'a, Self> {

        remove(self, names)

    }

    fn append(self, name: OuterType, data: RowView<'a, InnerType>) -> Append<'a, Self> {
        append(self, name, data)

    }
    fn concat<J>(self, other: J) -> Chain<Self, J>
        where J: Sized + Iterator<Item = (OuterType, RowView<'a, InnerType>)>
    {
        self.chain(other)
    }
    fn mapdf<B, F>(self, f: F) -> Map<Self, F>
        where F: FnMut(Self::DFItem) -> B
    {

        self.map(f)
    }
}

impl<'a, I> DFIter<'a> for Append<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{
    type DFItem = (OuterType, RowView<'a, InnerType>);
    fn select(self, names: Vec<OuterType>) -> Select<'a, Self> {

        select(self, names)
    }


    fn remove(self, names: Vec<OuterType>) -> Remove<'a, Self> {

        remove(self, names)

    }

    fn append(self, name: OuterType, data: RowView<'a, InnerType>) -> Append<'a, Self> {
        append(self, name, data)

    }
    fn concat<J>(self, other: J) -> Chain<Self, J>
        where J: Sized + Iterator<Item = (OuterType, RowView<'a, InnerType>)>
    {
        self.chain(other)
    }
    fn mapdf<B, F>(self, f: F) -> Map<Self, F>
        where F: FnMut(Self::DFItem) -> B
    {

        self.map(f)
    }
}




impl DataFrame {
    pub fn new<T: Clone>(data: Matrix<T>) -> DataFrame
        where InnerType: From<T>
    {
        let data: Matrix<InnerType> = data.mapv(InnerType::from);

        let columns: Vec<OuterType> = (0..data.shape()[1])
            .map(|x| OuterType::Str(x.to_string()))
            .collect();

        let index: Vec<OuterType> = (0..data.shape()[0])
            .map(|x| OuterType::Str(x.to_string()))
            .collect();

        DataFrame {
            data: data,
            columns: columns,
            index: index,
        }
    }


    pub fn columns<'a, T>(mut self, columns: &'a [T]) -> Result<DataFrame>
        where OuterType: From<&'a T>
    {
        if columns.len() != self.data.shape()[1] {
            return Err(ErrorKind::ColumnShapeMismatch.into());
        }
        self.columns = columns.iter()
            .map(|x| OuterType::from(x))
            .collect();
        Ok(self)
    }

    pub fn index<'a, T>(mut self, index: &'a [T]) -> Result<DataFrame>
        where OuterType: From<&'a T>
    {
        if index.len() != self.data.shape()[0] {
            return Err(ErrorKind::RowShapeMismatch.into());
        }
        self.index = index.iter()
            .map(|x| OuterType::from(x))
            .collect();
        Ok(self)
    }

    pub fn df_iter<'a>(&'a self, axis: Axis) -> DataFrameIterator<'a> {
        match axis {
            Axis(0) => {
                DataFrameIterator {
                    names: self.index.iter(),
                    data: self.data.axis_iter(Axis(0)),
                }
            }
            Axis(1) => {
                DataFrameIterator {
                    names: self.columns.iter(),
                    data: self.data.axis_iter(Axis(1)),
                }
            }
            _ => panic!(),

        }
    }
    pub fn select<'a>(&'a self,
                      ind: Vec<OuterType>,
                      axis: Axis)
                      -> Select<'a, DataFrameIterator<'a>> {
        match axis {
            Axis(0) => select(self.df_iter(Axis(0)), ind),
            Axis(1) => select(self.df_iter(Axis(1)), ind),
            _ => panic!(),

        }
    }


    pub fn remove<'a>(&'a self,
                      ind: Vec<OuterType>,
                      axis: Axis)
                      -> Remove<'a, DataFrameIterator<'a>> {
        match axis {
            Axis(0) => remove(self.df_iter(Axis(0)), ind),
            Axis(1) => remove(self.df_iter(Axis(1)), ind),
            _ => panic!(),

        }
    }

    pub fn append<'a>(&'a self,
                      name: OuterType,
                      data: RowView<'a, InnerType>,
                      axis: Axis)
                      -> Append<'a, DataFrameIterator<'a>> {
        match axis {
            Axis(0) => append(self.df_iter(Axis(0)), name, data),
            Axis(1) => append(self.df_iter(Axis(1)), name, data),
            _ => panic!(),

        }
    }
}



// To implement....?
// // parallelized join
// // parallelized concatenation
// // parallelized frequency counts
// // index dataframe?
// // sample rows
// // find/select
// // sort
// // statistics (mean, median, stdev)
// // print
//
// // statistics (mean, median, stdev)
// // print
