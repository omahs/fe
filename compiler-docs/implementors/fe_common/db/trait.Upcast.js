(function() {var implementors = {
"fe_analyzer":[["impl <a class=\"trait\" href=\"fe_common/db/trait.Upcast.html\" title=\"trait fe_common::db::Upcast\">Upcast</a>&lt;dyn <a class=\"trait\" href=\"fe_common/db/trait.SourceDb.html\" title=\"trait fe_common::db::SourceDb\">SourceDb</a>&gt; for <a class=\"struct\" href=\"fe_analyzer/db/struct.TestDb.html\" title=\"struct fe_analyzer::db::TestDb\">TestDb</a>"]],
"fe_codegen":[["impl Upcast&lt;dyn SourceDb&gt; for <a class=\"struct\" href=\"fe_codegen/db/struct.Db.html\" title=\"struct fe_codegen::db::Db\">Db</a>"],["impl Upcast&lt;dyn AnalyzerDb&gt; for <a class=\"struct\" href=\"fe_codegen/db/struct.Db.html\" title=\"struct fe_codegen::db::Db\">Db</a>"],["impl Upcast&lt;dyn MirDb&gt; for <a class=\"struct\" href=\"fe_codegen/db/struct.Db.html\" title=\"struct fe_codegen::db::Db\">Db</a>"]],
"fe_mir":[["impl Upcast&lt;dyn SourceDb&gt; for <a class=\"struct\" href=\"fe_mir/db/struct.NewDb.html\" title=\"struct fe_mir::db::NewDb\">NewDb</a>"],["impl Upcast&lt;dyn AnalyzerDb&gt; for <a class=\"struct\" href=\"fe_mir/db/struct.NewDb.html\" title=\"struct fe_mir::db::NewDb\">NewDb</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()