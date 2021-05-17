use crate::{
    collect::{Collectable, Collector},
    order::Order,
    *,
};

use std::fmt::{self, Write};

use super::{from_item::*, with::WithQuery, FromItem};

pub struct InsertQuery<T> {
    with: Option<WithQuery>,
    cols: Vec<T>,
    values: Vec<Box<dyn ValueExpr>>,
    returning: Option<Box<dyn Expr>>,
}

impl<T> Default for InsertQuery<T> {
    fn default() -> Self {
        InsertQuery {
            with: Default::default(),
            cols: Default::default(),
            values: Default::default(),
            returning: Default::default(),
        }
    }
}

impl<X> InsertQuery<X> {
    pub fn into<T: Table>(self) -> InsertQuery<T> {
        InsertQuery {
            cols: Vec::new(),
            with: self.with,
            ..InsertQuery::<T>::default()
        }
    }
}

impl<T: Table> InsertQuery<T> {
    pub fn cols<'a>(mut self, cols: impl IntoIterator<Item = &'a T>) -> Self {
        self.cols.extend(cols.into_iter().cloned());
        self
    }

    pub fn values<E>(mut self, exprs: impl IntoIterator<Item = E>) -> Self
    where
        E: ValueExpr + 'static,
    {
        self.values
            .extend(exprs.into_iter().map(|e| Box::new(e) as Box<dyn ValueExpr>));
        self
    }

    pub fn value<E>(mut self, expr: E) -> Self
    where
        E: ValueExpr + 'static,
    {
        self.values.push(Box::new(expr));
        self
    }

    pub fn returning<E>(mut self, expr: E) -> Self
    where
        E: Expr + 'static,
    {
        self.returning = Some(Box::new(expr));
        self
    }
}

impl<T: Table> Collectable for InsertQuery<T> {
    fn needs_wrapping(&self) -> bool {
        true
    }

    fn collect(&self, w: &mut dyn Write, t: &mut Collector) -> fmt::Result {
        use crate::expr::util::collect_delimited;

        if let Some(ref with) = self.with {
            with._collect(w, t)?;
            w.write_str(" ")?; // space before INSERT
        }

        w.write_str("INSERT INTO ")?;

        TableRef::<T>::new()._collect(w, t)?;

        // print column names without table prefix
        let mut cols = self.cols.iter();
        if let Some(col) = cols.next() {
            w.write_str(" (\"")?;
            w.write_str(col.name())?;

            for col in cols {
                w.write_str("\", \"")?;
                w.write_str(col.name())?;
            }
            w.write_str("\")")?;
        }

        if self.values.is_empty() {
            w.write_str(" DEFAULT VALUES")?;
        } else {
            if !self.cols.is_empty() {
                assert_eq!(
                    self.values.len(),
                    self.cols.len(),
                    "Columns and Values must be equal length!"
                );
            }

            w.write_str(" VALUES ")?;
            collect_delimited(&self.values, true, ", ", w, t)?;
        }

        if let Some(ref returning) = self.returning {
            w.write_str(" RETURNING ")?;
            returning._collect(w, t)?;
        }

        Ok(())
    }
}
