use account::AccountName;
use bytes::{Read, ReadError, Write, WriteError};
use eosio_macros::*;
use symbol::SymbolName;

eosio_name!(TableScope);

impl From<AccountName> for TableScope {
    fn from(account: AccountName) -> Self {
        let value: u64 = account.into();
        value.into()
    }
}

impl From<TableScope> for AccountName {
    fn from(scope: TableScope) -> Self {
        let value: u64 = scope.into();
        value.into()
    }
}

impl From<SymbolName> for TableScope {
    fn from(symbol: SymbolName) -> Self {
        let value: u64 = symbol.into();
        value.into()
    }
}

impl From<TableScope> for SymbolName {
    fn from(scope: TableScope) -> Self {
        let value: u64 = scope.into();
        value.into()
    }
}

pub trait TableRow: Read + Write {
    const NAME: u64;

    fn primary_key(&self) -> u64;

    fn secondary_keys(&self) -> [Option<&::table_secondary::SecondaryTableKey>; 16] {
        [None; 16]
    }

    fn table<C, S>(code: C, scope: S) -> ::table_primary::PrimaryTableIndex<Self>
    where
        C: Into<AccountName>,
        S: Into<TableScope>,
    {
        ::table_primary::PrimaryTableIndex::new(code, scope, Self::NAME)
    }
}

pub trait TableCursor<T>: IntoIterator
where
    T: TableRow,
{
    fn get(&self) -> Result<T, ReadError>;
    fn remove(&self) -> Result<T, ReadError>;
    fn update<P>(&self, payer: P, item: &T) -> Result<usize, WriteError>
    where
        P: Into<AccountName>;
}

pub trait TableIndex<'a, K, T>
where
    T: TableRow + 'a,
{
    type Cursor: TableCursor<T> + 'a;
    fn lower_bound<N>(&'a self, key: N) -> Option<Self::Cursor>
    where
        N: Into<K>;
    fn upper_bound<N>(&'a self, key: N) -> Option<Self::Cursor>
    where
        N: Into<K>;
    fn insert<P>(&'a self, payer: P, item: &'a T) -> Result<(), WriteError>
    where
        P: Into<AccountName>;
}

pub trait TableIterator: Iterator {
    fn asc(&self) -> Self;
    fn desc(&self) -> Self;
}
