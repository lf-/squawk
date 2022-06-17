use crate::{
    pg_version::PgVersion,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{AlterTableCmds, AlterTableType, RawStmt, Stmt};

#[must_use]
pub fn ban_drop_column(tree: &[RawStmt], _pg_version: Option<PgVersion>) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for cmd in &stmt.cmds {
                    let AlterTableCmds::AlterTableCmd(cmd) = cmd;
                    if cmd.subtype == AlterTableType::DropColumn {
                        errs.push(RuleViolation::new(
                            RuleViolationKind::BanDropColumn,
                            raw_stmt.into(),
                            None,
                        ));
                    }
                }
            }
            _ => continue,
        }
    }
    errs
}

#[cfg(test)]
mod test_rules {
    use crate::{check_sql, violations::RuleViolationKind};
    use insta::assert_debug_snapshot;

    #[test]
    fn test_drop_column() {
        let sql = r#"
ALTER TABLE "bar_tbl" DROP COLUMN "foo_col" CASCADE;
        "#;

        assert_debug_snapshot!(check_sql(
            sql,
            &[RuleViolationKind::PreferRobustStmts],
            None
        ));
    }
}
