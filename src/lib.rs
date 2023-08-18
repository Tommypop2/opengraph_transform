use std::collections::HashMap;
use std::sync::Arc;

use swc_core::common::errors::{ColorConfig, Handler};
use swc_core::common::{FileName, SourceMap, GLOBALS};
use swc_core::ecma::ast::{
    EsVersion, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue, JSXExpr, JSXExprContainer,
    JSXOpeningElement, MemberExpr, MemberProp, Str,
};
use swc_core::ecma::parser::{EsConfig, Syntax};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{
            ArrowExpr, BindingIdent, BlockStmt, BlockStmtOrExpr, CallExpr, Callee, Decl, Expr,
            ExprOrSpread, Ident, ImportDecl, ImportNamedSpecifier, ImportSpecifier, JSXElement,
            JSXElementChild, JSXElementName, Module, ModuleDecl, ModuleExportName, ModuleItem, Pat,
            Program, ReturnStmt, Stmt, VarDecl, VarDeclKind, VarDeclarator,
        },
        utils::prepend_stmt,
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
};
use wasm_bindgen::prelude::wasm_bindgen;

pub struct TransformVisitor {
    elements: Vec<Box<JSXElement>>,
    imports: HashMap<String, (Ident, String)>,
}
impl Default for TransformVisitor {
    fn default() -> Self {
        Self {
            elements: vec![],
            imports: Default::default(),
        }
    }
}
impl TransformVisitor {
    // Copied almost verbatim from swc-plugin-jsx-dom-expressions
    pub fn insert_imports(&mut self, module: &mut Module) {
        let mut entries = self.imports.drain().collect::<Vec<_>>();
        entries.sort_by(|(a, _), (b, _)| a.cmp(b));
        for (name, (val, from)) in entries {
            prepend_stmt(
                &mut module.body,
                ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                    specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
                        local: val,
                        imported: Some(ModuleExportName::Ident(Ident::new(name.into(), DUMMY_SP))),
                        span: DUMMY_SP,
                        is_type_only: false,
                    })],
                    src: Box::new(Str {
                        span: DUMMY_SP,
                        value: from.into(),
                        raw: None,
                    }),
                    span: DUMMY_SP,
                    type_only: false,
                    asserts: None,
                })),
            );
        }
    }
    pub fn insert_functions(&mut self, module: &mut Module) {
        if self.elements.len() < 1 {
            return;
        }
        let stmt = Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: Default::default(),
            kind: VarDeclKind::Const,
            declare: false,
            decls: vec![VarDeclarator {
                span: Default::default(),
                name: Pat::Ident(BindingIdent {
                    id: Ident {
                        span: Default::default(),
                        sym: "img".into(),
                        optional: false,
                    },
                    type_ann: None,
                }),
                init: Some(Box::new(Expr::Call(CallExpr {
                    span: Default::default(),
                    callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                        span: Default::default(),
                        sym: {
                            // Add to imports
                            self.imports.insert(
                                "server$".into(),
                                (
                                    Ident {
                                        span: Default::default(),
                                        optional: false,
                                        sym: "server$".into(),
                                    },
                                    "solid-start/server".into(),
                                ),
                            );
                            "server$".into()
                        },
                        optional: false,
                    }))),
                    args: vec![ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Arrow(ArrowExpr {
                            span: Default::default(),
                            params: vec![],
                            is_async: false,
                            is_generator: false,
                            return_type: None,
                            type_params: None,
                            body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                                span: Default::default(),
                                stmts: vec![Stmt::Return(ReturnStmt {
                                    span: Default::default(),
                                    arg: Some(Box::new(Expr::Call(CallExpr {
                                        span: Default::default(),
                                        callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                                            span: Default::default(),
                                            sym: {
                                                self.imports.insert(
                                                    "createOpenGraphImage".into(),
                                                    (
                                                        Ident {
                                                            span: Default::default(),
                                                            optional: false,
                                                            sym: "createOpenGraphImage$".into(),
                                                        },
                                                        "@solid-mediakit/open-graph".into(),
                                                    ),
                                                );
                                                "createOpenGraphImage$".into()
                                            },
                                            optional: false,
                                        }))),
                                        args: vec![ExprOrSpread {
                                            spread: None,
                                            expr: Box::new(Expr::JSXElement(
                                                self.elements[0].to_owned(),
                                            )),
                                        }],
                                        type_args: None,
                                    }))),
                                })],
                            })),
                        })),
                    }],
                    type_args: None,
                }))),
                definite: false,
            }],
        })));
        prepend_stmt(&mut module.body, ModuleItem::Stmt(stmt))
    }
}
impl VisitMut for TransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
    fn visit_mut_jsx_element(&mut self, n: &mut JSXElement) {
        if let JSXElementName::Ident(i) = &n.opening.name {
            if i.sym.to_string() == "OpenGraph" {
                // Only grab first child that is an element for now
                let children: &Vec<JSXElementChild> = &n.children;
                for child in children {
                    if let JSXElementChild::JSXElement(element) = child {
                        self.elements.push(element.to_owned());
                        break;
                    }
                }
                *n = JSXElement {
                    span: Default::default(),
                    opening: JSXOpeningElement {
                        self_closing: true,
                        type_args: None,
                        span: Default::default(),
                        name: JSXElementName::Ident(Ident {
                            span: Default::default(),
                            sym: "img".into(),
                            optional: false,
                        }),
                        attrs: vec![JSXAttrOrSpread::JSXAttr(JSXAttr {
                            span: Default::default(),
                            name: JSXAttrName::Ident(Ident {
                                span: Default::default(),
                                sym: "src".into(),
                                optional: false,
                            }),
                            value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                                span: Default::default(),
                                expr: JSXExpr::Expr(Box::new(Expr::Member(MemberExpr {
                                    span: DUMMY_SP,
                                    obj: Box::new(Expr::Ident(Ident {
                                        span: DUMMY_SP,
                                        optional: false,
                                        sym: "img".into(),
                                    })),
                                    prop: MemberProp::Ident(Ident {
                                        span: DUMMY_SP,
                                        optional: false,
                                        sym: "src".into(),
                                    }),
                                }))),
                            })),
                        })],
                    },
                    children: vec![],
                    closing: None,
                };
            }
        }
    }
    fn visit_mut_module(&mut self, module: &mut Module) {
        module.visit_mut_children_with(self);
        self.insert_functions(module);
        self.insert_imports(module);
    }
}

// #[plugin_transform]
// pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
//     program.fold_with(&mut as_folder(TransformVisitor::default()))
// }
#[wasm_bindgen]
pub fn main(code: String, id: String) -> String {
    let cm: Arc<SourceMap> = Arc::<SourceMap>::default();
    let handler: Handler =
        Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let compiler: swc::Compiler = swc::Compiler::new(cm.clone());

    let fm: Arc<swc_core::common::SourceFile> = cm.new_source_file(FileName::Custom(id), code);
    GLOBALS.set(&Default::default(), || {
        let result = compiler.parse_js(
            fm,
            &handler,
            EsVersion::EsNext,
            Syntax::Es(EsConfig {
                jsx: true,
                ..Default::default()
            }),
            swc::config::IsModule::Bool(true),
            None,
        );

        // let inter = result
        //     .unwrap()
        //     .fold_with(&mut as_folder(TransformVisitor::default()));

        let out = compiler.print(
            &result.unwrap(),
            None,
            None,
            false,
            EsVersion::EsNext,
            swc::config::SourceMapsConfig::Bool(false),
            &Default::default(),
            None,
            false,
            None,
            false,
            false,
            "",
        );
        out.unwrap().code
    })
}
