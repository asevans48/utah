
use util::types::*;
use std::iter::{Iterator, Chain};
use adapters::aggregate::*;
use adapters::transform::*;
use adapters::process::*;
use dataframe::{DataFrame, MutableDataFrame, DataFrameIterator, MutableDataFrameIterator};
use std::hash::Hash;
use std::fmt::Debug;
use util::error::*;
use adapters::join::*;
use std::ops::{Add, Sub, Mul, Div};
use num::traits::{One, Zero};
use std::collections::BTreeMap;


pub trait Num
    : Add<Output = Self> +
      Div<Output = Self> +
      Sub<Output = Self> +
      Mul<Output = Self> +
      Empty<Self> +
      One +
      Zero +
      Clone +
      Debug +
      PartialEq +
      Default
    {}
impl<T> Num for T
    where T: Add<Output = T> + Div<Output = T> + Sub<Output = T> + Mul<Output = T> + Empty<T> + One + Zero + Clone + Debug + PartialEq +  Default
{
}

pub trait Identifier: Hash + Eq + Ord + From<String> + Clone + Debug + Default {}

impl<T> Identifier for T where T: Hash + Eq + Ord + From<String> + Clone + Debug + Default {}


pub trait Empty<T> {
    fn empty() -> T;
    fn is_empty(&self) -> bool;
}


pub trait MixedDataframeConstructor<'a, I, T, S>
    where I: Iterator<Item = RowView<'a, T>> + Clone,
          T: 'a + Num,
          S: Identifier,
          Self: Sized
{
    fn new<U: Clone + Debug + Ord>(data: BTreeMap<U, Row<T>>) -> Self
        where S: From<U>,
              U: 'a;
    fn index<U: Clone + Ord>(self, index: &'a [U]) -> Result<Self> where S: From<U>;
    fn columns<U: Clone + Ord>(self, columns: &'a [U]) -> Result<Self> where S: From<U>;
    fn from_array(data: Row<T>, axis: UtahAxis) -> Self;
    fn df_iter(&'a self, axis: UtahAxis) -> DataFrameIterator<'a, T, S>;
    fn df_iter_mut(&'a mut self, axis: UtahAxis) -> MutableDataFrameIterator<'a, T, S>;
}


pub trait Constructor<'a, T, S>
    where T: 'a + Num,
          S: Identifier,
          Self: Sized
{
    fn new<U: Clone + Debug>(data: Matrix<U>) -> Self where T: From<U>;
    fn index<U: Clone>(self, index: &'a [U]) -> Result<Self> where S: From<U>;
    fn columns<U: Clone>(self, columns: &'a [U]) -> Result<Self> where S: From<U>;
    fn from_array<U: Clone>(data: Row<U>, axis: UtahAxis) -> Self where T: From<U>;
    fn df_iter(&'a self, axis: UtahAxis) -> DataFrameIterator<'a, T, S>;
    fn df_iter_mut(&'a mut self, axis: UtahAxis) -> MutableDataFrameIterator<'a, T, S>;
}


pub trait Operations<'a, T, S>
    where T: 'a + Num,
          S: Identifier
{
    fn shape(self) -> (usize, usize);
    fn select<U: ?Sized>(&'a self,
                         names: &'a [&'a U],
                         axis: UtahAxis)
                         -> Select<'a, DataFrameIterator<'a, T, S>, T, S>
        where S: From<&'a U>;
    fn remove<U: ?Sized>(&'a self,
                         names: &'a [&'a U],
                         axis: UtahAxis)
                         -> Remove<'a, DataFrameIterator<'a, T, S>, T, S>
        where S: From<&'a U>;
    fn append<U: ?Sized>(&'a mut self,
                         name: &'a U,
                         data: RowView<'a, T>,
                         axis: UtahAxis)
                         -> Append<'a, DataFrameIterator<'a, T, S>, T, S>
        where S: From<&'a U>;
    fn inner_left_join(&'a self,
                       other: &'a DataFrame<T, S>)
                       -> InnerJoin<'a, DataFrameIterator<'a, T, S>, T, S>;
    fn outer_left_join(&'a self,
                       other: &'a DataFrame<T, S>)
                       -> OuterJoin<'a, DataFrameIterator<'a, T, S>, T, S>;
    fn inner_right_join(&'a self,
                        other: &'a DataFrame<T, S>)
                        -> InnerJoin<'a, DataFrameIterator<'a, T, S>, T, S>;
    fn outer_right_join(&'a self,
                        other: &'a DataFrame<T, S>)
                        -> OuterJoin<'a, DataFrameIterator<'a, T, S>, T, S>;
    fn concat
        (&'a self,
         other: &'a DataFrame<T, S>,
         axis: UtahAxis)
         -> Concat<'a, Chain<DataFrameIterator<'a, T, S>, DataFrameIterator<'a, T, S>>, T, S>;
    fn sumdf(&'a mut self, axis: UtahAxis) -> Sum<'a, DataFrameIterator<'a, T, S>, T, S>;
    fn map<F, B>(&'a mut self,
                 f: F,
                 axis: UtahAxis)
                 -> MapDF<'a, T, S, DataFrameIterator<'a, T, S>, F, B>
        where F: Fn(&T) -> B,
              for<'r> F: Fn(&InnerType) -> B;
    fn mean(&'a mut self, axis: UtahAxis) -> Mean<'a, DataFrameIterator<'a, T, S>, T, S>;
    fn maxdf(&'a mut self, axis: UtahAxis) -> Max<'a, DataFrameIterator<'a, T, S>, T, S>;
    fn min(&'a mut self, axis: UtahAxis) -> Min<'a, DataFrameIterator<'a, T, S>, T, S>;
    fn stdev(&'a self, axis: UtahAxis) -> Stdev<'a, DataFrameIterator<'a, T, S>, T, S>;
    fn impute(&'a mut self,
              strategy: ImputeStrategy,
              axis: UtahAxis)
              -> Impute<'a, MutableDataFrameIterator<'a, T, S>, T, S>;
}

pub trait Aggregate<'a, T, S>
    where T: Num,
          S: Identifier
{
    fn sumdf(self) -> Sum<'a, Self, T, S> where Self: Sized + Iterator<Item = (S, RowView<'a, T>)>;

    fn maxdf(self) -> Max<'a, Self, T, S> where Self: Sized + Iterator<Item = (S, RowView<'a, T>)>;

    fn mindf(self) -> Min<'a, Self, T, S> where Self: Sized + Iterator<Item = (S, RowView<'a, T>)>;

    fn mean(self) -> Mean<'a, Self, T, S> where Self: Sized + Iterator<Item = (S, RowView<'a, T>)>;

    fn stdev(self) -> Stdev<'a, Self, T, S> where Self: Sized + Iterator<Item = (S, RowView<'a, T>)>;
}

pub trait Process<'a, T, S>
    where T: Num,
          S: Identifier
{
    fn impute(self, strategy: ImputeStrategy) -> Impute<'a, Self, T, S>
        where Self: Sized + Iterator<Item = (S, RowViewMut<'a, T>)>;
    fn to_mut_df(self) -> MutableDataFrame<'a, T, S>
        where Self: Sized + Iterator<Item = (S, RowViewMut<'a, T>)>;
}

pub trait Transform<'a, T, S>
    where T: Num + 'a,
          S: Identifier
{
    fn select<U: ?Sized>(self, names: &'a [&'a U]) -> Select<'a, Self, T, S>
        where Self: Sized + Iterator<Item = (S, RowView<'a, T>)> + Clone,
              S: From<&'a U>,
              T: 'a;
    fn remove<U: ?Sized>(self, names: &'a [&'a U]) -> Remove<'a, Self, T, S>
        where Self: Sized + Iterator<Item = (S, RowView<'a, T>)> + Clone,
              S: From<&'a U>,
              T: 'a;
    fn append<U: ?Sized>(self, name: &'a U, data: RowView<'a, T>) -> Append<'a, Self, T, S>
        where Self: Sized + Iterator<Item = (S, RowView<'a, T>)> + Clone,
              S: From<&'a U>,
              T: 'a;


    fn mapdf<F, B>(self, f: F) -> MapDF<'a, T, S, Self, F, B>
        where Self: Sized + Iterator<Item = (S, RowView<'a, T>)> + Clone,
              F: Fn(&T) -> B;
}



pub trait ToDataFrame<'a, I, T, S>
    where T: Num + 'a,
          S: Identifier
{
    fn as_df(self) -> Result<DataFrame<T, S>> where Self: Sized + Iterator<Item = I>;
    fn as_matrix(self) -> Result<Matrix<T>> where Self: Sized + Iterator<Item = I>;
    fn as_array(self) -> Result<Row<T>> where Self: Sized + Iterator<Item = I>;
}