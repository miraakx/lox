    mod assignment {
        use super::test;
        #[test]
        fn prefix_operator() {
            test("./lox_test/assignment/prefix_operator.lox");
        }
        #[test]
        fn associativity() {
            test("./lox_test/assignment/associativity.lox");
        }
        #[test]
        fn undefined() {
            test("./lox_test/assignment/undefined.lox");
        }
        #[test]
        fn to_this() {
            test("./lox_test/assignment/to_this.lox");
        }
        #[test]
        fn global() {
            test("./lox_test/assignment/global.lox");
        }
        #[test]
        fn grouping() {
            test("./lox_test/assignment/grouping.lox");
        }
        #[test]
        fn syntax() {
            test("./lox_test/assignment/syntax.lox");
        }
        #[test]
        fn infix_operator() {
            test("./lox_test/assignment/infix_operator.lox");
        }
        #[test]
        fn local() {
            test("./lox_test/assignment/local.lox");
        }
    }
    mod operator {
        use super::test;
        #[test]
        fn not() {
            test("./lox_test/operator/not.lox");
        }
        #[test]
        fn less_or_equal_nonnum_num() {
            test("./lox_test/operator/less_or_equal_nonnum_num.lox");
        }
        #[test]
        fn add_nil_nil() {
            test("./lox_test/operator/add_nil_nil.lox");
        }
        #[test]
        fn less_or_equal_num_nonnum() {
            test("./lox_test/operator/less_or_equal_num_nonnum.lox");
        }
        #[test]
        fn less_nonnum_num() {
            test("./lox_test/operator/less_nonnum_num.lox");
        }
        #[test]
        fn multiply() {
            test("./lox_test/operator/multiply.lox");
        }
        #[test]
        fn equals_method() {
            test("./lox_test/operator/equals_method.lox");
        }
        #[test]
        fn multiply_nonnum_num() {
            test("./lox_test/operator/multiply_nonnum_num.lox");
        }
        #[test]
        fn subtract() {
            test("./lox_test/operator/subtract.lox");
        }
        #[test]
        fn not_class() {
            test("./lox_test/operator/not_class.lox");
        }
        #[test]
        fn subtract_num_nonnum() {
            test("./lox_test/operator/subtract_num_nonnum.lox");
        }
        #[test]
        fn less_num_nonnum() {
            test("./lox_test/operator/less_num_nonnum.lox");
        }
        #[test]
        fn greater_num_nonnum() {
            test("./lox_test/operator/greater_num_nonnum.lox");
        }
        #[test]
        fn negate_nonnum() {
            test("./lox_test/operator/negate_nonnum.lox");
        }
        #[test]
        fn not_equals() {
            test("./lox_test/operator/not_equals.lox");
        }
        #[test]
        fn add_bool_nil() {
            test("./lox_test/operator/add_bool_nil.lox");
        }
        #[test]
        fn divide_nonnum_num() {
            test("./lox_test/operator/divide_nonnum_num.lox");
        }
        #[test]
        fn subtract_nonnum_num() {
            test("./lox_test/operator/subtract_nonnum_num.lox");
        }
        #[test]
        fn negate() {
            test("./lox_test/operator/negate.lox");
        }
        #[test]
        fn greater_or_equal_num_nonnum() {
            test("./lox_test/operator/greater_or_equal_num_nonnum.lox");
        }
        #[test]
        fn add() {
            test("./lox_test/operator/add.lox");
        }
        #[test]
        fn greater_or_equal_nonnum_num() {
            test("./lox_test/operator/greater_or_equal_nonnum_num.lox");
        }
        #[test]
        fn divide() {
            test("./lox_test/operator/divide.lox");
        }
        #[test]
        fn add_bool_string() {
            test("./lox_test/operator/add_bool_string.lox");
        }
        #[test]
        fn add_string_nil() {
            test("./lox_test/operator/add_string_nil.lox");
        }
        #[test]
        fn equals() {
            test("./lox_test/operator/equals.lox");
        }
        #[test]
        fn greater_nonnum_num() {
            test("./lox_test/operator/greater_nonnum_num.lox");
        }
        #[test]
        fn add_num_nil() {
            test("./lox_test/operator/add_num_nil.lox");
        }
        #[test]
        fn comparison() {
            test("./lox_test/operator/comparison.lox");
        }
        #[test]
        fn equals_class() {
            test("./lox_test/operator/equals_class.lox");
        }
        #[test]
        fn divide_num_nonnum() {
            test("./lox_test/operator/divide_num_nonnum.lox");
        }
        #[test]
        fn multiply_num_nonnum() {
            test("./lox_test/operator/multiply_num_nonnum.lox");
        }
        #[test]
        fn add_bool_num() {
            test("./lox_test/operator/add_bool_num.lox");
        }
    }
    mod field {
        use super::test;
        #[test]
        fn many() {
            test("./lox_test/field/many.lox");
        }
        #[test]
        fn method() {
            test("./lox_test/field/method.lox");
        }
        #[test]
        fn set_on_nil() {
            test("./lox_test/field/set_on_nil.lox");
        }
        #[test]
        fn method_binds_this() {
            test("./lox_test/field/method_binds_this.lox");
        }
        #[test]
        fn call_nonfunction_field() {
            test("./lox_test/field/call_nonfunction_field.lox");
        }
        #[test]
        fn get_on_function() {
            test("./lox_test/field/get_on_function.lox");
        }
        #[test]
        fn set_on_num() {
            test("./lox_test/field/set_on_num.lox");
        }
        #[test]
        fn get_on_nil() {
            test("./lox_test/field/get_on_nil.lox");
        }
        #[test]
        fn set_on_function() {
            test("./lox_test/field/set_on_function.lox");
        }
        #[test]
        fn undefined() {
            test("./lox_test/field/undefined.lox");
        }
        #[test]
        fn call_function_field() {
            test("./lox_test/field/call_function_field.lox");
        }
        #[test]
        fn get_on_string() {
            test("./lox_test/field/get_on_string.lox");
        }
        #[test]
        fn get_on_bool() {
            test("./lox_test/field/get_on_bool.lox");
        }
        #[test]
        fn on_instance() {
            test("./lox_test/field/on_instance.lox");
        }
        #[test]
        fn set_on_class() {
            test("./lox_test/field/set_on_class.lox");
        }
        #[test]
        fn set_on_bool() {
            test("./lox_test/field/set_on_bool.lox");
        }
        #[test]
        fn get_and_set_method() {
            test("./lox_test/field/get_and_set_method.lox");
        }
        #[test]
        fn set_on_string() {
            test("./lox_test/field/set_on_string.lox");
        }
        #[test]
        fn get_on_class() {
            test("./lox_test/field/get_on_class.lox");
        }
        #[test]
        fn set_evaluation_order() {
            test("./lox_test/field/set_evaluation_order.lox");
        }
        #[test]
        fn get_on_num() {
            test("./lox_test/field/get_on_num.lox");
        }
    }
    mod method {
        use super::test;
        #[test]
        fn missing_arguments() {
            test("./lox_test/method/missing_arguments.lox");
        }
        #[test]
        fn print_bound_method() {
            test("./lox_test/method/print_bound_method.lox");
        }
        #[test]
        fn too_many_parameters() {
            test("./lox_test/method/too_many_parameters.lox");
        }
        #[test]
        fn extra_arguments() {
            test("./lox_test/method/extra_arguments.lox");
        }
        #[test]
        fn arity() {
            test("./lox_test/method/arity.lox");
        }
        #[test]
        fn empty_block() {
            test("./lox_test/method/empty_block.lox");
        }
        #[test]
        fn too_many_arguments() {
            test("./lox_test/method/too_many_arguments.lox");
        }
        #[test]
        fn not_found() {
            test("./lox_test/method/not_found.lox");
        }
        #[test]
        fn refer_to_name() {
            test("./lox_test/method/refer_to_name.lox");
        }
    }
    mod this {
        use super::test;
        #[test]
        fn this_at_top_level() {
            test("./lox_test/this/this_at_top_level.lox");
        }
        #[test]
        fn nested_closure() {
            test("./lox_test/this/nested_closure.lox");
        }
        #[test]
        fn nested_class() {
            test("./lox_test/this/nested_class.lox");
        }
        #[test]
        fn this_in_method() {
            test("./lox_test/this/this_in_method.lox");
        }
        #[test]
        fn closure() {
            test("./lox_test/this/closure.lox");
        }
        #[test]
        fn this_in_top_level_function() {
            test("./lox_test/this/this_in_top_level_function.lox");
        }
    }
    mod regression {
        use super::test;
        #[test]
        fn regression_1() {
            test("./lox_test/regression/regression_1.lox");
        }
        #[test]
        fn regression_2() {
            test("./lox_test/regression/regression_2.lox");
        }
    }
    mod constructor {
        use super::test;
        #[test]
        fn missing_arguments() {
            test("./lox_test/constructor/missing_arguments.lox");
        }
        #[test]
        fn early_return() {
            test("./lox_test/constructor/early_return.lox");
        }
        #[test]
        fn extra_arguments() {
            test("./lox_test/constructor/extra_arguments.lox");
        }
        #[test]
        fn init_not_method() {
            test("./lox_test/constructor/init_not_method.lox");
        }
        #[test]
        fn arguments() {
            test("./lox_test/constructor/arguments.lox");
        }
        #[test]
        fn return_value() {
            test("./lox_test/constructor/return_value.lox");
        }
        #[test]
        fn return_in_nested_function() {
            test("./lox_test/constructor/return_in_nested_function.lox");
        }
        #[test]
        fn default_arguments() {
            test("./lox_test/constructor/default_arguments.lox");
        }
        #[test]
        fn call_init_explicitly() {
            test("./lox_test/constructor/call_init_explicitly.lox");
        }
        #[test]
        fn call_init_early_return() {
            test("./lox_test/constructor/call_init_early_return.lox");
        }
        #[test]
        fn default() {
            test("./lox_test/constructor/default.lox");
        }
    }
    mod expressions {
        use super::test;
        #[test]
        fn evaluate() {
            test("./lox_test/expressions/evaluate.lox");
        }
        #[test]
        fn parse() {
            test("./lox_test/expressions/parse.lox");
        }
    }
    mod function {
        use super::test;
        #[test]
        fn missing_arguments() {
            test("./lox_test/function/missing_arguments.lox");
        }
        #[test]
        fn too_many_parameters() {
            test("./lox_test/function/too_many_parameters.lox");
        }
        #[test]
        fn extra_arguments() {
            test("./lox_test/function/extra_arguments.lox");
        }
        #[test]
        fn nested_call_with_arguments() {
            test("./lox_test/function/nested_call_with_arguments.lox");
        }
        #[test]
        fn mutual_recursion() {
            test("./lox_test/function/mutual_recursion.lox");
        }
        #[test]
        fn parameters() {
            test("./lox_test/function/parameters.lox");
        }
        #[test]
        fn print() {
            test("./lox_test/function/print.lox");
        }
        #[test]
        fn too_many_arguments() {
            test("./lox_test/function/too_many_arguments.lox");
        }
        #[test]
        fn local_recursion() {
            test("./lox_test/function/local_recursion.lox");
        }
        #[test]
        fn body_must_be_block() {
            test("./lox_test/function/body_must_be_block.lox");
        }
        #[test]
        fn empty_body() {
            test("./lox_test/function/empty_body.lox");
        }
        #[test]
        fn local_mutual_recursion() {
            test("./lox_test/function/local_mutual_recursion.lox");
        }
        #[test]
        fn recursion() {
            test("./lox_test/function/recursion.lox");
        }
        #[test]
        fn missing_comma_in_parameters() {
            test("./lox_test/function/missing_comma_in_parameters.lox");
        }
    }
    mod comments {
        use super::test;
        #[test]
        fn line_at_eof() {
            test("./lox_test/comments/line_at_eof.lox");
        }
        #[test]
        fn unicode() {
            test("./lox_test/comments/unicode.lox");
        }
        #[test]
        fn only_line_comment_and_line() {
            test("./lox_test/comments/only_line_comment_and_line.lox");
        }
        #[test]
        fn only_line_comment() {
            test("./lox_test/comments/only_line_comment.lox");
        }
    }
    mod string {
        use super::test;
        #[test]
        fn error_after_multiline() {
            test("./lox_test/string/error_after_multiline.lox");
        }
        #[test]
        fn multiline() {
            test("./lox_test/string/multiline.lox");
        }
        #[test]
        fn unterminated() {
            test("./lox_test/string/unterminated.lox");
        }
        #[test]
        fn literals() {
            test("./lox_test/string/literals.lox");
        }
    }
    mod for_keyword {
        use super::test;
        #[test]
        fn scope() {
            test("./lox_test/for/scope.lox");
        }
        #[test]
        fn statement_initializer() {
            test("./lox_test/for/statement_initializer.lox");
        }
        #[test]
        fn return_inside() {
            test("./lox_test/for/return_inside.lox");
        }
        #[test]
        fn statement_condition() {
            test("./lox_test/for/statement_condition.lox");
        }
        #[test]
        fn statement_increment() {
            test("./lox_test/for/statement_increment.lox");
        }
        #[test]
        fn closure_in_body() {
            test("./lox_test/for/closure_in_body.lox");
        }
        #[test]
        fn var_in_body() {
            test("./lox_test/for/var_in_body.lox");
        }
        #[test]
        fn class_in_body() {
            test("./lox_test/for/class_in_body.lox");
        }
        #[test]
        fn syntax() {
            test("./lox_test/for/syntax.lox");
        }
        #[test]
        fn return_closure() {
            test("./lox_test/for/return_closure.lox");
        }
        #[test]
        fn fun_in_body() {
            test("./lox_test/for/fun_in_body.lox");
        }
    }
    mod inheritance {
        use super::test;
        #[test]
        fn set_fields_from_base_class() {
            test("./lox_test/inheritance/set_fields_from_base_class.lox");
        }
        #[test]
        fn inherit_methods() {
            test("./lox_test/inheritance/inherit_methods.lox");
        }
        #[test]
        fn parenthesized_superclass() {
            test("./lox_test/inheritance/parenthesized_superclass.lox");
        }
        #[test]
        fn constructor() {
            test("./lox_test/inheritance/constructor.lox");
        }
        #[test]
        fn inherit_from_function() {
            test("./lox_test/inheritance/inherit_from_function.lox");
        }
        #[test]
        fn inherit_from_number() {
            test("./lox_test/inheritance/inherit_from_number.lox");
        }
        #[test]
        fn inherit_from_nil() {
            test("./lox_test/inheritance/inherit_from_nil.lox");
        }
    }
    mod scanning {
        use super::test;
        #[test]
        fn punctuators() {
            test("./lox_test/scanning/punctuators.lox");
        }
        #[test]
        fn numbers() {
            test("./lox_test/scanning/numbers.lox");
        }
        #[test]
        fn strings() {
            test("./lox_test/scanning/strings.lox");
        }
        #[test]
        fn keywords() {
            test("./lox_test/scanning/keywords.lox");
        }
        #[test]
        fn whitespace() {
            test("./lox_test/scanning/whitespace.lox");
        }
        #[test]
        fn identifiers() {
            test("./lox_test/scanning/identifiers.lox");
        }
    }
    mod limit {
        use super::test;
        #[test]
        fn no_reuse_constants() {
            test("./lox_test/limit/no_reuse_constants.lox");
        }
        #[test]
        fn stack_overflow() {
            test("./lox_test/limit/stack_overflow.lox");
        }
        #[test]
        fn too_many_constants() {
            test("./lox_test/limit/too_many_constants.lox");
        }
        #[test]
        fn too_many_upvalues() {
            test("./lox_test/limit/too_many_upvalues.lox");
        }
        #[test]
        fn loop_too_large() {
            test("./lox_test/limit/loop_too_large.lox");
        }
        #[test]
        fn too_many_locals() {
            test("./lox_test/limit/too_many_locals.lox");
        }
    }
    mod closure {
        use super::test;
        #[test]
        fn close_over_later_variable() {
            test("./lox_test/closure/close_over_later_variable.lox");
        }
        #[test]
        fn unused_later_closure() {
            test("./lox_test/closure/unused_later_closure.lox");
        }
        #[test]
        fn assign_to_closure() {
            test("./lox_test/closure/assign_to_closure.lox");
        }
        #[test]
        fn reference_closure_multiple_times() {
            test("./lox_test/closure/reference_closure_multiple_times.lox");
        }
        #[test]
        fn close_over_method_parameter() {
            test("./lox_test/closure/close_over_method_parameter.lox");
        }
        #[test]
        fn assign_to_shadowed_later() {
            test("./lox_test/closure/assign_to_shadowed_later.lox");
        }
        #[test]
        fn reuse_closure_slot() {
            test("./lox_test/closure/reuse_closure_slot.lox");
        }
        #[test]
        fn nested_closure() {
            test("./lox_test/closure/nested_closure.lox");
        }
        #[test]
        fn open_closure_in_function() {
            test("./lox_test/closure/open_closure_in_function.lox");
        }
        #[test]
        fn unused_closure() {
            test("./lox_test/closure/unused_closure.lox");
        }
        #[test]
        fn shadow_closure_with_local() {
            test("./lox_test/closure/shadow_closure_with_local.lox");
        }
        #[test]
        fn closed_closure_in_function() {
            test("./lox_test/closure/closed_closure_in_function.lox");
        }
        #[test]
        fn close_over_function_parameter() {
            test("./lox_test/closure/close_over_function_parameter.lox");
        }
    }
    mod nil {
        use super::test;
        #[test]
        fn literal() {
            test("./lox_test/nil/literal.lox");
        }
    }
    mod others {
        use super::test;
        #[test]
        fn precedence() {
            test("./lox_test/others/precedence.lox");
        }
        #[test]
        fn unexpected_character() {
            test("./lox_test/others/unexpected_character.lox");
        }
        #[test]
        fn empty_file() {
            test("./lox_test/others/empty_file.lox");
        }
    }
    mod while_keyword {
        use super::test;
        #[test]
        fn return_inside() {
            test("./lox_test/while/return_inside.lox");
        }
        #[test]
        fn closure_in_body() {
            test("./lox_test/while/closure_in_body.lox");
        }
        #[test]
        fn var_in_body() {
            test("./lox_test/while/var_in_body.lox");
        }
        #[test]
        fn class_in_body() {
            test("./lox_test/while/class_in_body.lox");
        }
        #[test]
        fn syntax() {
            test("./lox_test/while/syntax.lox");
        }
        #[test]
        fn return_closure() {
            test("./lox_test/while/return_closure.lox");
        }
        #[test]
        fn fun_in_body() {
            test("./lox_test/while/fun_in_body.lox");
        }
    }
    mod return_keyword {
        use super::test;
        #[test]
        fn after_while() {
            test("./lox_test/return/after_while.lox");
        }
        #[test]
        fn in_method() {
            test("./lox_test/return/in_method.lox");
        }
        #[test]
        fn return_nil_if_no_value() {
            test("./lox_test/return/return_nil_if_no_value.lox");
        }
        #[test]
        fn after_else() {
            test("./lox_test/return/after_else.lox");
        }
        #[test]
        fn at_top_level() {
            test("./lox_test/return/at_top_level.lox");
        }
        #[test]
        fn in_function() {
            test("./lox_test/return/in_function.lox");
        }
        #[test]
        fn after_if() {
            test("./lox_test/return/after_if.lox");
        }
    }
    mod super_keyword {
        use super::test;
        #[test]
        fn missing_arguments() {
            test("./lox_test/super/missing_arguments.lox");
        }
        #[test]
        fn call_same_method() {
            test("./lox_test/super/call_same_method.lox");
        }
        #[test]
        fn super_in_top_level_function() {
            test("./lox_test/super/super_in_top_level_function.lox");
        }
        #[test]
        fn this_in_superclass_method() {
            test("./lox_test/super/this_in_superclass_method.lox");
        }
        #[test]
        fn extra_arguments() {
            test("./lox_test/super/extra_arguments.lox");
        }
        #[test]
        fn super_in_inherited_method() {
            test("./lox_test/super/super_in_inherited_method.lox");
        }
        #[test]
        fn super_at_top_level() {
            test("./lox_test/super/super_at_top_level.lox");
        }
        #[test]
        fn no_superclass_call() {
            test("./lox_test/super/no_superclass_call.lox");
        }
        #[test]
        fn no_superclass_method() {
            test("./lox_test/super/no_superclass_method.lox");
        }
        #[test]
        fn parenthesized() {
            test("./lox_test/super/parenthesized.lox");
        }
        #[test]
        fn call_other_method() {
            test("./lox_test/super/call_other_method.lox");
        }
        #[test]
        fn indirectly_inherited() {
            test("./lox_test/super/indirectly_inherited.lox");
        }
        #[test]
        fn constructor() {
            test("./lox_test/super/constructor.lox");
        }
        #[test]
        fn super_without_name() {
            test("./lox_test/super/super_without_name.lox");
        }
        #[test]
        fn closure() {
            test("./lox_test/super/closure.lox");
        }
        #[test]
        fn super_in_closure_in_inherited_method() {
            test("./lox_test/super/super_in_closure_in_inherited_method.lox");
        }
        #[test]
        fn reassign_superclass() {
            test("./lox_test/super/reassign_superclass.lox");
        }
        #[test]
        fn no_superclass_bind() {
            test("./lox_test/super/no_superclass_bind.lox");
        }
        #[test]
        fn bound_method() {
            test("./lox_test/super/bound_method.lox");
        }
        #[test]
        fn super_without_dot() {
            test("./lox_test/super/super_without_dot.lox");
        }
    }
    mod logical_operator {
        use super::test;
        #[test]
        fn and_truth() {
            test("./lox_test/logical_operator/and_truth.lox");
        }
        #[test]
        fn or_truth() {
            test("./lox_test/logical_operator/or_truth.lox");
        }
        #[test]
        fn or() {
            test("./lox_test/logical_operator/or.lox");
        }
        #[test]
        fn and() {
            test("./lox_test/logical_operator/and.lox");
        }
    }
    mod variable {
        use super::test;
        #[test]
        fn scope_reuse_in_different_blocks() {
            test("./lox_test/variable/scope_reuse_in_different_blocks.lox");
        }
        #[test]
        fn shadow_local() {
            test("./lox_test/variable/shadow_local.lox");
        }
        #[test]
        fn redefine_global() {
            test("./lox_test/variable/redefine_global.lox");
        }
        #[test]
        fn use_local_in_initializer() {
            test("./lox_test/variable/use_local_in_initializer.lox");
        }
        #[test]
        fn undefined_local() {
            test("./lox_test/variable/undefined_local.lox");
        }
        #[test]
        fn duplicate_parameter() {
            test("./lox_test/variable/duplicate_parameter.lox");
        }
        #[test]
        fn redeclare_global() {
            test("./lox_test/variable/redeclare_global.lox");
        }
        #[test]
        fn duplicate_local() {
            test("./lox_test/variable/duplicate_local.lox");
        }
        #[test]
        fn use_this_as_var() {
            test("./lox_test/variable/use_this_as_var.lox");
        }
        #[test]
        fn shadow_global() {
            test("./lox_test/variable/shadow_global.lox");
        }
        #[test]
        fn early_bound() {
            test("./lox_test/variable/early_bound.lox");
        }
        #[test]
        fn in_middle_of_block() {
            test("./lox_test/variable/in_middle_of_block.lox");
        }
        #[test]
        fn local_from_method() {
            test("./lox_test/variable/local_from_method.lox");
        }
        #[test]
        fn collide_with_parameter() {
            test("./lox_test/variable/collide_with_parameter.lox");
        }
        #[test]
        fn uninitialized() {
            test("./lox_test/variable/uninitialized.lox");
        }
        #[test]
        fn use_global_in_initializer() {
            test("./lox_test/variable/use_global_in_initializer.lox");
        }
        #[test]
        fn unreached_undefined() {
            test("./lox_test/variable/unreached_undefined.lox");
        }
        #[test]
        fn use_nil_as_var() {
            test("./lox_test/variable/use_nil_as_var.lox");
        }
        #[test]
        fn in_nested_block() {
            test("./lox_test/variable/in_nested_block.lox");
        }
        #[test]
        fn shadow_and_local() {
            test("./lox_test/variable/shadow_and_local.lox");
        }
        #[test]
        fn use_false_as_var() {
            test("./lox_test/variable/use_false_as_var.lox");
        }
        #[test]
        fn undefined_global() {
            test("./lox_test/variable/undefined_global.lox");
        }
    }
    mod bool {
        use super::test;
        #[test]
        fn not() {
            test("./lox_test/bool/not.lox");
        }
        #[test]
        fn equality() {
            test("./lox_test/bool/equality.lox");
        }
    }
    mod if_keyword {
        use super::test;
        #[test]
        fn class_in_else() {
            test("./lox_test/if/class_in_else.lox");
        }
        #[test]
        fn var_in_then() {
            test("./lox_test/if/var_in_then.lox");
        }
        #[test]
        fn class_in_then() {
            test("./lox_test/if/class_in_then.lox");
        }
        #[test]
        fn else_keyword() {
            test("./lox_test/if/else.lox");
        }
        #[test]
        fn fun_in_else() {
            test("./lox_test/if/fun_in_else.lox");
        }
        #[test]
        fn dangling_else() {
            test("./lox_test/if/dangling_else.lox");
        }
        #[test]
        fn fun_in_then() {
            test("./lox_test/if/fun_in_then.lox");
        }
        #[test]
        fn truth() {
            test("./lox_test/if/truth.lox");
        }
        #[test]
        fn var_in_else() {
            test("./lox_test/if/var_in_else.lox");
        }
        #[test]
        fn if_keyword() {
            test("./lox_test/if/if.lox");
        }
    }
    mod print {
        use super::test;
        #[test]
        fn missing_argument() {
            test("./lox_test/print/missing_argument.lox");
        }
    }
    mod class {
        use super::test;
        #[test]
        fn empty() {
            test("./lox_test/class/empty.lox");
        }
        #[test]
        fn local_reference_self() {
            test("./lox_test/class/local_reference_self.lox");
        }
        #[test]
        fn reference_self() {
            test("./lox_test/class/reference_self.lox");
        }
        #[test]
        fn inherit_self() {
            test("./lox_test/class/inherit_self.lox");
        }
        #[test]
        fn inherited_method() {
            test("./lox_test/class/inherited_method.lox");
        }
        #[test]
        fn local_inherit_self() {
            test("./lox_test/class/local_inherit_self.lox");
        }
        #[test]
        fn local_inherit_other() {
            test("./lox_test/class/local_inherit_other.lox");
        }
    }
    mod block {
        use super::test;
        #[test]
        fn scope() {
            test("./lox_test/block/scope.lox");
        }
        #[test]
        fn empty() {
            test("./lox_test/block/empty.lox");
        }
    }
    mod call {
        use super::test;
        #[test]
        fn nil() {
            test("./lox_test/call/nil.lox");
        }
        #[test]
        fn string() {
            test("./lox_test/call/string.lox");
        }
        #[test]
        fn bool() {
            test("./lox_test/call/bool.lox");
        }
        #[test]
        fn object() {
            test("./lox_test/call/object.lox");
        }
        #[test]
        fn num() {
            test("./lox_test/call/num.lox");
        }
    }
    mod number {
        use super::test;
        #[test]
        fn leading_dot() {
            test("./lox_test/number/leading_dot.lox");
        }
        #[test]
        fn literals() {
            test("./lox_test/number/literals.lox");
        }
        #[test]
        fn decimal_point_at_eof() {
            test("./lox_test/number/decimal_point_at_eof.lox");
        }
        #[test]
        fn trailing_dot() {
            test("./lox_test/number/trailing_dot.lox");
        }
        #[test]
        fn nan_equality() {
            test("./lox_test/number/nan_equality.lox");
        }
    }
