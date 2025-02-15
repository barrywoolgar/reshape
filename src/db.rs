use postgres::{types::ToSql, NoTls, Row};

pub trait Conn {
    fn run(&mut self, query: &str) -> anyhow::Result<()>;
    fn query(&mut self, query: &str) -> anyhow::Result<Vec<Row>>;
    fn query_with_params(
        &mut self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> anyhow::Result<Vec<Row>>;
    fn transaction(&mut self) -> anyhow::Result<Transaction>;
}

pub struct DbConn {
    client: postgres::Client,
}

impl DbConn {
    pub fn connect(config: &postgres::Config) -> anyhow::Result<DbConn> {
        let client = config.connect(NoTls)?;
        Ok(DbConn { client })
    }
}

impl Conn for DbConn {
    fn run(&mut self, query: &str) -> anyhow::Result<()> {
        self.client.batch_execute(query)?;
        Ok(())
    }

    fn query(&mut self, query: &str) -> anyhow::Result<Vec<Row>> {
        let rows = self.client.query(query, &[])?;
        Ok(rows)
    }

    fn query_with_params(
        &mut self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> anyhow::Result<Vec<Row>> {
        let rows = self.client.query(query, params)?;
        Ok(rows)
    }

    fn transaction(&mut self) -> anyhow::Result<Transaction> {
        let transaction = self.client.transaction()?;
        Ok(Transaction { transaction })
    }
}

pub struct Transaction<'a> {
    transaction: postgres::Transaction<'a>,
}

impl Transaction<'_> {
    pub fn commit(self) -> anyhow::Result<()> {
        self.transaction.commit()?;
        Ok(())
    }
}

impl Conn for Transaction<'_> {
    fn run(&mut self, query: &str) -> anyhow::Result<()> {
        self.transaction.batch_execute(query)?;
        Ok(())
    }

    fn query(&mut self, query: &str) -> anyhow::Result<Vec<Row>> {
        let rows = self.transaction.query(query, &[])?;
        Ok(rows)
    }

    fn query_with_params(
        &mut self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> anyhow::Result<Vec<Row>> {
        let rows = self.transaction.query(query, params)?;
        Ok(rows)
    }

    fn transaction(&mut self) -> anyhow::Result<Transaction> {
        let transaction = self.transaction.transaction()?;
        Ok(Transaction { transaction })
    }
}
