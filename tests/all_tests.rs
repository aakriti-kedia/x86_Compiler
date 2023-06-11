mod infra;

// Your tests go here!
success_tests! {
    {
        name: cycle_print1,
        file: "cycle_print1.snek",
        expected: "((array <cyclic>), (array <cyclic>))\n((array <cyclic>), (array <cyclic>))",
    },
    {
        name: cycle_print2,
        file: "cycle_print2.snek",
        expected: "(((array <cyclic>)), 20)\n(((array <cyclic>), 20))\n((40, 45, (array <cyclic>)), 20, 30)\n(40, 45, ((array <cyclic>), 20, 30))\n(40, 45, ((array <cyclic>), 20, 30))",
    },
    {
        name: cycle_print3,
        file: "cycle_print3.snek",
        expected: "(((array <cyclic>), 40), (20, (array <cyclic>)))\n(20, (((array <cyclic>), 40), (array <cyclic>)))\n(((array <cyclic>), (20, (array <cyclic>))), 40)\n(((array <cyclic>), (20, (array <cyclic>))), 40)",
    },
    {
        name: cycle_equal1,
        file: "cycle_equal1.snek",
        expected: "((array <cyclic>), (array <cyclic>))\n((array <cyclic>), (array <cyclic>))\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue",
    },
    {
        name: cycle_equal2,
        file: "cycle_equal2.snek",
        expected: "((30, (array <cyclic>)), 20)\n(30, ((array <cyclic>), 20))\n((30, ((array <cyclic>), 20)), 20)\n(30, ((30, (array <cyclic>)), 20))\n((30, ((30, (array <cyclic>)), 20)), 20)\n(30, ((30, ((array <cyclic>), 20)), 20))\nfalse\nfalse\nfalse\nfalse\nfalse\nfalse\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue",
    },
    {
        name: cycle_equal3,
        file: "cycle_equal3.snek",
        expected: "(((array <cyclic>), 40), (20, (array <cyclic>)))\n(20, (((array <cyclic>), 40), (array <cyclic>)))\n(((array <cyclic>), (20, (array <cyclic>))), 40)\n(20, (((array <cyclic>), 40), (20, (array <cyclic>))))\ntrue\ntrue\ntrue\nfalse\ntrue\nfalse\nfalse",
    },
    {
        name: reference_eq1,
        file: "reference_eq1.snek",
        expected: "true\nfalse\nfalse\nfalse",
    },
    {
        name: structural_reference_equality,
        file: "structural_reference_equality.snek",
        expected: "true\ntrue\nfalse\ntrue\ntrue\nfalse\nfalse\nfalse\nfalse\ntrue\ntrue\nfalse\nfalse\nfalse\nfalse",
    },
    {
        name: cyclic_print_equality,
        file: "cyclic_print_equality.snek",
        expected: "(10, (10, (array <cyclic>), 30), 30)\n(10, (10, (array <cyclic>), 30), 30)\n(10, (10, (10, (array <cyclic>), 30), 30), 30)\n(10, (10, (10, (array <cyclic>), 30), 30))\ntrue\ntrue\nfalse",
    },
    {
        name: simple_examples,
        file: "simple_examples.snek",
        expected: "(10, nil, 12)\n(3, 4, 3, 5, 5)\n(1, 2)\n(1, (2, 3), (4, (5, 6)), (7, (8, 9)))\n13\n5\n(4, (5, 6))\n(2, 6, 44, 5)\n(2, 6, 4, 55)\n(1, (2, 3), 5, (7, (8, 9)))\n(1, (2, 3), (4, 5, 6, 7, 8), (7, (8, 9)))\n(1, (2, 3), (41, (42, 43), (44, 45, (46, (47, 48)))), (7, (8, 9)))\n7\n7",
    },
    {
        name: points,
        file: "points.snek",
        expected: "(5, 9)\n(20, 30)\n(5, 10)\n(4, 8)\n(11, 8)\n(11, 8)\n(11, 12)\n(-1, 12)\n(-1, 8)\n(-1, -2)\n(-1, -2)",
    },
    {
        name: bst,
        file: "bst.snek",
        expected: "(10, nil, nil)\n(10, (8, nil, nil), nil)\n(10, (8, nil, nil), nil)\n10\n(10, (8, nil, nil), nil)\ntrue\n(10, (8, nil, nil), nil)\n11\n(10, (8, nil, nil), nil)\nnil\nfalse\n(10, nil, nil)\n(10, (8, nil, nil), nil)\n(10, (8, nil, nil), (12, nil, nil))\n(10, (8, (6, nil, nil), nil), (12, nil, nil))\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, nil, nil))\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, (11, nil, nil), nil))\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, (11, nil, nil), (15, nil, nil)))\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, (11, nil, nil), (15, nil, nil)))\n9\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, (11, nil, nil), (15, nil, nil)))\n(8, (6, nil, nil), (9, nil, nil))\n(9, nil, nil)\ntrue\n(10, nil, nil)\n(10, (8, nil, nil), nil)\n(10, (8, nil, nil), (12, nil, nil))\n(10, (8, (6, nil, nil), nil), (12, nil, nil))\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, nil, nil))\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, (11, nil, nil), nil))\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, (11, nil, nil), (15, nil, nil)))\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, (11, nil, nil), (15, nil, nil)))\n14\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, (11, nil, nil), (15, nil, nil)))\n(12, (11, nil, nil), (15, nil, nil))\n(15, nil, nil)\nnil\nfalse\nfalse",
    },
    {
        name: points_fun,
        file: "points_fun.snek",
        expected: "(5, 9)",
    },
    {
        name: bst_search_with_nil_found,
        file: "bst_search_with_nil_found.snek",
        expected: "(10, 8, 12, 6, 9, 11, 14, nil, nil, nil, nil, nil, nil, 13, 15)\n15\n1\n3\n7\n15\ntrue",
    },
    {
        name: bst_search_with_nil_not_found,
        file: "bst_search_with_nil_not_found.snek",
        expected: "(10, 8, 12, 6, 9, 11, 14, nil, nil, nil, nil, nil, nil, 13, 15)\n20\n1\n3\n7\n15\n31\nfalse",
    },
    {
        name: array_with_nil,
        file: "array_with_nil.snek",
        expected: "(10, nil, 12)",
    },
    {
        name: array_initialized_odd,
        file: "array_initialized_odd.snek",
        expected: "(3, 4, 3, 5, 5)",
    },
    {
        name: array_initialized_even,
        file: "array_initialized_even.snek",
        expected: "(1, 2)",
    },
    {
        name: nested_array_init,
        file: "nested_array_init.snek",
        expected: "(1, (2, 3), (4, (5, 6)), (7, (8, 9)))",
    },
    {
        name: get_array_index_odd,
        file: "get_array_index_odd.snek",
        expected: "13",
    },
    {
        name: get_array_index_even,
        file: "get_array_index_even.snek",
        expected: "5",
    },
    {
        name: nested_array_get,
        file: "nested_array_get.snek",
        expected: "(4, (5, 6))",
    },
    {
        name: set_array_index_odd,
        file: "set_array_index_odd.snek",
        expected: "(2, 6, 44, 5)",
    },
    {
        name: set_array_index_even,
        file: "set_array_index_even.snek",
        expected: "(2, 6, 4, 55)",
    },
    {
        name: nested_array_set_num,
        file: "nested_array_set_num.snek",
        expected: "(1, (2, 3), 5, (7, (8, 9)))",
    },
    {
        name: nested_array_set_array,
        file: "nested_array_set_array.snek",
        expected: "(1, (2, 3), (4, 5, 6, 7, 8), (7, (8, 9)))",
    },
    {
        name: nested_array_set_nested_array,
        file: "nested_array_set_nested_array.snek",
        expected: "(1, (2, 3), (41, (42, 43), (44, 45, (46, (47, 48)))), (7, (8, 9)))",
    },
    {
        name: set_cyclic,
        file: "set_cyclic.snek",
        expected: "(43, 22, (75, 26, 34, (12, 34, 56, 78, 90, (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, (array <cyclic>), 13, 14, 15))))\n(43, 22, (75, 26, 34, (12, 34, 56, 78, 90, (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, (array <cyclic>), 13, 14, 15))))",
    },
    {
        name: let_array,
        file: "let_array.snek",
        expected: "10\n30\n30",
    },
    {
        name: get_array_size,
        file: "get_array_size.snek",
        expected: "7",
    },
    {
        name: get_size_empty_array,
        file: "get_size_empty_array.snek",
        expected: "0",
    },
    {
        name: bst_search,
        file: "bst_search.snek",
        expected: "(10, (8, (6, nil, nil), (9, nil, nil)), (12, (11, nil, nil), (15, nil, nil)))\n15\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, (11, nil, nil), (15, nil, nil)))\n(12, (11, nil, nil), (15, nil, nil))\n(15, nil, nil)\ntrue\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, (11, nil, nil), (15, nil, nil)))\n13\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, (11, nil, nil), (15, nil, nil)))\n(12, (11, nil, nil), (15, nil, nil))\n(15, nil, nil)\nnil\nfalse\nfalse",
    },
    {
        name: bst_insert,
        file: "bst_insert.snek",
        expected: "(10, nil, nil)\n(10, (8, nil, nil), nil)\n(10, (8, nil, nil), (12, nil, nil))\n(10, (8, (6, nil, nil), nil), (12, nil, nil))\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, nil, nil))\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, (11, nil, nil), nil))\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, (11, nil, nil), (15, nil, nil)))\n(10, (8, (6, nil, nil), (9, nil, nil)), (12, (11, nil, nil), (15, nil, nil)))",
    },
    {
        name: is_nil,
        file: "is_nil.snek",
        expected: "false\nfalse\ntrue\ntrue",
    },
    {
        name: check_type_match,
        file: "check_type_match.snek",
        expected: "false\nfalse\ntrue\nfalse\ntrue\ntrue\nfalse\nfalse",
    },
    {
        name: fact,
        file: "fact.snek",
        input: "10",
        expected: "3628800",
    },
    {
        name: fib_rec,
        file: "fib_rec.snek",
        expected: "55",
    },
    {
        name: multi_level_nesting,
        file: "multi_level_nesting.snek",
        expected: "7",
    },
    {
        name: multiple_args_nested_odd,
        file: "multiple_args_nested_odd.snek",
        expected: "3",
    },
    {
        name: multiple_args_nested_even,
        file: "multiple_args_nested_even.snek",
        expected: "2",
    },
    {
        name: ackermann_rec,
        file: "ackermann_rec.snek",
        expected: "29",
    },
    {
        name: fact_rec,
        file: "fact_rec.snek",
        input: "10",
        expected: "3628800",
    },
    {
        name: double,
        file: "double.snek",
        input: "10",
        expected: "20",
    },
    {
        name: five_arg,
        file: "five_arg.snek",
        expected: "10",
    },
    {
        name: nested_five_arg_3,
        file: "nested_five_arg.snek",
        expected: "30",
    },
    {
        name: even_nested_fun,
        file: "even_nested_fun.snek",
        expected: "11",
    },
    {
        name: odd_nested_fun,
        file: "odd_nested_fun.snek",
        expected: "10",
    },
    {
        name: fun_no_args,
        file: "fun_no_args.snek",
        expected: "5",
    },
    {
        name: nested_double,
        file: "nested_double.snek",
        input: "10",
        expected: "20",
    },
    {
        name: even_odd_1,
        file: "even_odd.snek",
        input: "10",
        expected: "10\ntrue\ntrue",
    },
    {
        name: even_odd_2,
        file: "even_odd.snek",
        input: "9",
        expected: "9\nfalse\nfalse",
    },
    {
        name: add1_sub1_int_input_10,
        file: "add1_sub1_input.snek",
        input: "10",
        expected: "10"
    },
    {
        name: simple_if_then_else,
        file: "simple_if_then_else.snek",
        expected: "500"
    },
    {
        name: factorial_5,
        file: "factorial_from_input.snek",
        input: "5",
        expected: "120"
    },
    {
        name: power_of_2_calculator_7,
        file: "power_of_2_calculator.snek",
        input: "7",
        expected: "128"
    },
    {
        name: complex_set,
        file: "complex_set.snek",
        input: "3",
        expected: "14"
    },
    {
        name: print_num,
        file: "print_num.snek",
        expected: "5\n5",
    },
    {
        name: print_add,
        file: "print_add.snek",
        expected: "8\n8",
    },
    // {
    //     name: block_print_input_num,
    //     file: "block_print.snek",
    //     input: 12,
    //     expected: "37\n12",
    // },
    // {
    //     name: block_print_input_none,
    //     file: "block_print.snek",
    //     expected: "37\nfalse",
    // },
    {
        name: input_num,
        file: "input.snek",
        input: "4",
        expected: "4",
    },
    // {
    //     name: input_num_min,
    //     file: "input.snek",
    //     input: "-9223372036854775808",
    //     expected: "-9223372036854775808",
    // },
    // {
    //     name: input_num_max,
    //     file: "input.snek",
    //     input: "9223372036854775807",
    //     expected: "9223372036854775807",
    // },
    {
        name: input_num_min,
        file: "input.snek",
        input: "-4611686018427387904",
        expected: "-4611686018427387904",
    },
    {
        name: input_num_max,
        file: "input.snek",
        input: "4611686018427387903",
        expected: "4611686018427387903",
    },
    {
        name: input_true,
        file: "input.snek",
        input: "true",
        expected: "true",
    },
    {
        name: input_false,
        file: "input.snek",
        input: "false",
        expected: "false",
    },
    {
        name: input_none,
        file: "input.snek",
        expected: "false",
    },
    {
        name: eq_num_num_true,
        file: "eq_num.snek",
        input: "3",
        expected: "true",
    },
    {
        name: eq_num_num_false,
        file: "eq_num.snek",
        input: "4",
        expected: "false",
    },

    {
        name: g_num_num_true,
        file: "g_num.snek",
        input: "5",
        expected: "true",
    },
    {
        name: g_num_num_false,
        file: "g_num.snek",
        input: "2",
        expected: "false",
    },

    {
        name: ge_num_num_true,
        file: "ge_num.snek",
        input: "3",
        expected: "true",
    },
    {
        name: ge_num_num_false,
        file: "ge_num.snek",
        input: "2",
        expected: "false",
    },

    {
        name: l_num_num_true,
        file: "l_num.snek",
        input: "2",
        expected: "true",
    },
    {
        name: l_num_num_false,
        file: "l_num.snek",
        input: "4",
        expected: "false",
    },
    {
        name: le_num_num_true,
        file: "le_num.snek",
        input: "3",
        expected: "true",
    },
    {
        name: le_num_num_false,
        file: "le_num.snek",
        input: "4",
        expected: "false",
    },
    {
        name: true_sexp,
        file: "true_sexp.snek",
        expected: "true",
    },
    {
        name: false_sexp,
        file: "false_sexp.snek",
        expected: "false",
    },
    {
        name: sub_pos_neg,
        file: "sub_pos_neg.snek",
        expected: "10",
    },
    {
        name: sub_neg_pos,
        file: "sub_neg_pos.snek",
        expected: "-10",
    },
    {
        name: sub_neg_neg_pos_res,
        file: "sub_neg_neg_pos_res.snek",
        expected: "20",
    },
    {
        name: sub_neg_neg_neg_res,
        file: "sub_neg_neg_neg_res.snek",
        expected: "-10",
    },
    {
        name: nested_sub,
        file: "nested_sub.snek",
        expected: "-76",
    },
    {
        name: nested_sub_neg,
        file: "nested_sub_neg.snek",
        expected: "-116",
    },

    {
        name: add_pos_pos,
        file: "add_pos_pos.snek",
        expected: "20",
    },
    {
        name: add_pos_neg,
        file: "add_pos_neg.snek",
        expected: "-10",
    },
    {
        name: add_neg_pos,
        file: "add_neg_pos.snek",
        expected: "10",
    },
    {
        name: add_neg_neg,
        file: "add_neg_neg.snek",
        expected: "-20",
    },
    {
        name: nested_add,
        file: "nested_add.snek",
        expected: "124",
    },
    {
        name: nested_add_neg,
        file: "nested_add_neg.snek",
        expected: "-16",
    },

    {
        name: times_pos_pos,
        file: "times_pos_pos.snek",
        expected: "75",
    },
    {
        name: times_pos_neg,
        file: "times_pos_neg.snek",
        expected: "-75",
    },
    {
        name: times_neg_neg,
        file: "times_neg_neg.snek",
        expected: "75",
    },

    {
        name: num,
        file: "num.snek",
        expected: "5",
    },

    {
        name: nested_binop1,
        file: "nested_binop1.snek",
        expected: "-60",
    },
    {
        name: nested_binop2,
        file: "nested_binop2.snek",
        expected: "10",
    },
    {
        name: nested_binop3,
        file: "nested_binop3.snek",
        expected: "-40",
    },
    {
        name: nested_binop4,
        file: "nested_binop4.snek",
        expected: "22",
    },
    {
        name: nested_binop5,
        file: "nested_binop5.snek",
        expected: "-10",
    },
    {
        name: nested_binop6,
        file: "nested_binop6.snek",
        expected: "10",
    },
    {
        name: nested_binop_unop,
        file: "nested_binop_unop.snek",
        expected: "22",
    },
    {
        name: let_one_var,
        file: "let_one_var.snek",
        expected: "5",
    },
    {
        name: let_two_var,
        file: "let_two_var.snek",
        expected: "6",
    },
    {
        name: let_two_unary,
        file: "let_two_unary.snek",
        expected: "12",
    },
    {
        name: let_two_var_add,
        file: "let_two_var_add.snek",
        expected: "20",
    },
    {
        name: let_two_unary_bin,
        file: "let_two_unary_bin.snek",
        expected: "21",
    },
    {
        name: let_add1,
        file: "let_add1.snek",
        expected: "6",
    },
    {
        name: let_add,
        file: "let_add.snek",
        expected: "15",
    },
    {
        name: input_add_1,
        file: "input_add_1.snek",
        input: "4",
        expected: "5",
    },
    {
        name: false_val,
        file: "false_val.snek",
        expected: "false",
    },
    {
        name: compare_g_false,
        file: "compare_g_false.snek",
        expected: "false",
    },
    {
        name: compare_g_true,
        file: "compare_g_true.snek",
        expected: "true",
    },
    {
        name: compare_ge_false,
        file: "compare_ge_false.snek",
        expected: "false",
    },
    {
        name: compare_ge_true,
        file: "compare_ge_true.snek",
        expected: "true",
    },
    {
        name: compare_l_false,
        file: "compare_l_false.snek",
        expected: "false",
    },
    {
        name: compare_l_true,
        file: "compare_l_true.snek",
        expected: "true",
    },
    {
        name: compare_le_false,
        file: "compare_le_false.snek",
        expected: "false",
    },
    {
        name: compare_le_true,
        file: "compare_le_true.snek",
        expected: "true",
    },
    {
        name: input_compare_1,
        file: "input_compare.snek",
        input: "2",
        expected: "false",
    },
    {
        name: input_compare_2,
        file: "input_compare.snek",
        input: "10",
        expected: "true",
    },
    {
        name: loop_simple,
        file: "loop_simple.snek",
        expected: "2",
    },
    {
        name: block_set,
        file: "block_set.snek",
        expected: "12",
    },
    {
        name: block_set_last_val,
        file: "block_set_two_var.snek",
        expected: "10",
    },
    {
        name: set_simple,
        file: "set_simple.snek",
        expected: "10",
    },
    {
        name: is_num_true,
        file: "is_num_true.snek",
        expected: "true",
    },
    {
        name: is_num_false,
        file: "is_num_false.snek",
        expected: "false",
    },
    {
        name: is_bool_true,
        file: "is_bool_true.snek",
        expected: "true",
    },
    {
        name: is_bool_false,
        file: "is_bool_false.snek",
        expected: "false",
    },
    {
        name: input_valid,
        file: "input.snek",
        input: "10",
        expected: "10",
    },
    {
        name: input_negative,
        file: "input.snek",
        input: "-10",
        expected: "-10",
    },
    {
        name: if_true,
        file: "if_true.snek",
        expected: "true",
    },
    {
        name: if_false,
        file: "if_false.snek",
        expected: "false",
    },
    {
        name: if_compare_bool_true,
        file: "if_compare_bool_true.snek",
        expected: "true",
    },
    {
        name: if_compare_bool_false,
        file: "if_compare_bool_false.snek",
        expected: "false",
    },
    {
        name: if_num,
        file: "if_num.snek",
        expected: "1",
    },
    {
        name: loop_eg,
        file: "loop_eg.snek",
        expected: "-6",
    },
    {
        name: if_num_2,
        file: "if_num_2.snek",
        expected: "true",
    },
    {
        name: multiply,
        file: "mult.snek",
        input: "1537228672809129301",
        expected: "4611686018427387903",
    },
    {
        name: multiply2,
        file: "mult.snek",
        input: "1537228672809129300",
        expected: "4611686018427387900",
    },
    {
        name: multiply3,
        file: "mult.snek",
        input: "-1537228672809129301",
        expected: "-4611686018427387903",
    },
    {
        name: multiply_neg,
        file: "mult.snek",
        input: "-4",
        expected: "-12",
    },
    {
        name: multiply_neg_neg,
        file: "mult_neg.snek",
        input: "-5",
        expected: "20",
    },
    {
        name: evaluation_order,
        file: "evaluation_order.snek",
        expected: "3",
    }
}

runtime_error_tests! {
    {
        name: error_tag,
        file: "error-tag.snek",
        input: "false",
        expected: "invalid",
    },
    {
        name: get_array_out_of_bounds,
        file: "error-bounds.snek",
        expected: "index_out_of_bounds_error",
    },
    {
        name: get_array_index_boolean,
        file: "get_array_index_boolean.snek",
        expected: "invalid",
    },
    {
        name: get_index_nil_addr,
        file: "get_index_nil_addr.snek",
        expected: "invalid",
    },
    {
        name: get_index_num,
        file: "get_index_num.snek",
        expected: "invalid",
    },
    {
        name: set_array_out_of_bounds,
        file: "set_array_index_out_of_bounds.snek",
        expected: "index_out_of_bounds_error",
    },
    {
        name: invalid_argument,
        file: "invalid_argument.snek",
        expected: "invalid argument",
    },
    {
        name: input_compare_3,
        file: "input_compare.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: add1_sub1_int_input_true,
        file: "add1_sub1_input.snek",
        input: "true",
        expected: "invalid argument"
    },
    {
        name: add1_sub1_int_input_default_false,
        file: "add1_sub1_input.snek",
        expected: "invalid argument"
    },
    {
        name: add1_sub1_int_input_default_overflow_after_one_op,
        file: "add1_sub1_input.snek",
        input: "-4611686018427387904",
        expected: "overflow"
    },
    {
        name: add1_sub1_int_input_default_overflow_from_input,
        file: "add1_sub1_input.snek",
        input: "-4611686018427387905",
        expected: "Invalid"
    },
    {
        name: factorial_1000,
        file: "factorial_from_input.snek",
        input: "1000",
        expected: "overflow"
    },
    {
        name: power_of_2_calculator_1000,
        file: "power_of_2_calculator.snek",
        input: "1000",
        expected: "overflow"
    },
    {
        name: power_of_2_calculator_boolean_input,
        file: "power_of_2_calculator.snek",
        input: "true",
        expected: "invalid"
    },
    {
        name: compare_invalid,
        file: "compare_invalid.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: compare_invalid_eq,
        file: "compare_invalid_eq.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_num_bool_eq,
        file: "eq_num.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: invalid_num_bool_eq2,
        file: "eq_num.snek",
        input: "false",
        expected: "invalid argument",
    },
    {
        name: invalid_num_true_eq,
        file: "eq_true.snek",
        input: "1",
        expected: "invalid argument",
    },
    {
        name: invalid_num_false_eq,
        file: "eq_false.snek",
        input: "3",
        expected: "invalid argument",
    },

    {
        name: invalid_num_bool_g,
        file: "g_num.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: invalid_num_bool_g2,
        file: "g_num.snek",
        input: "false",
        expected: "invalid argument",
    },
    {
        name: invalid_num_true_g,
        file: "g_true.snek",
        input: "1",
        expected: "invalid argument",
    },
    {
        name: invalid_num_false_g,
        file: "g_false.snek",
        input: "3",
        expected: "invalid argument",
    },

    {
        name: invalid_num_bool_ge,
        file: "ge_num.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: invalid_num_bool_ge2,
        file: "ge_num.snek",
        input: "false",
        expected: "invalid argument",
    },
    {
        name: invalid_num_true_ge,
        file: "ge_true.snek",
        input: "1",
        expected: "invalid argument",
    },
    {
        name: invalid_num_false_ge,
        file: "ge_false.snek",
        input: "3",
        expected: "invalid argument",
    },

    {
        name: invalid_num_bool_l,
        file: "l_num.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: invalid_num_bool_l2,
        file: "l_num.snek",
        input: "false",
        expected: "invalid argument",
    },
    {
        name: invalid_num_true_l,
        file: "l_true.snek",
        input: "1",
        expected: "invalid argument",
    },
    {
        name: invalid_num_false_l,
        file: "l_false.snek",
        input: "3",
        expected: "invalid argument",
    },

    {
        name: invalid_num_bool_le,
        file: "le_num.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: invalid_num_bool_le2,
        file: "le_num.snek",
        input: "false",
        expected: "invalid argument",
    },
    {
        name: invalid_num_true_le,
        file: "le_true.snek",
        input: "1",
        expected: "invalid argument",
    },
    {
        name: invalid_num_false_le,
        file: "le_false.snek",
        input: "3",
        expected: "invalid argument",
    },


    {
        name: input_invalid,
        file: "input.snek",
        input: "10a",
        expected: "Invalid",
    },
    {
        name: input_invalid2,
        file: "input.snek",
        input: "invalid",
        expected: "Invalid",
    },
    {
        name: input_invalid_overflow,
        file: "input.snek",
        input: "36893488147419103000",
        expected: "Invalid",
    },
    {
        name: input_overflow,
        file: "input.snek",
        input: "9223372036854775807",
        expected: "Invalid",
    },
    {
        name: input_overflow_2,
        file: "input.snek",
        input: "-4611686018427387905",
        expected: "Invalid",
    },
    {
        name: invalid_if_condition,
        file: "invalid_if_condition.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_type_add,
        file: "invalid_type_add.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_type_sub,
        file: "invalid_type_sub.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_type_mult,
        file: "invalid_type_mult.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_type_add1,
        file: "invalid_type_add1.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_type_sub1,
        file: "invalid_type_sub1.snek",
        expected: "invalid argument",
    },
    {
        name: overflow_add,
        file: "overflow_add.snek",
        expected: "overflow",
    },
    {
        name: overflow_sub,
        file: "overflow_sub.snek",
        expected: "overflow",
    },
    {
        name: overflow_mult,
        file: "overflow_mult.snek",
        expected: "overflow",
    },
    {
        name: overflow_add1,
        file: "overflow_add1.snek",
        expected: "overflow",
    },
    {
        name: overflow_sub1,
        file: "overflow_sub1.snek",
        expected: "overflow",
    },
    {
        name: mult_error,
        file: "mult.snek",
        input: "1537228672809129302",
        expected: "overflow",
    },
    {
        name: multiply_neg_error_2,
        file: "mult.snek",
        input: "-1537228672809129302",
        expected: "overflow",
    },
}

static_error_tests! {
    {
        name: incorrect_arg,
        file: "incorrect_arg.snek",
        expected: "Invalid",
    },
    {
        name: array_input,
        file: "error3.snek",
        expected: "keyword",
    },
    {
        name: fun_def_input,
        file: "fun_def_input.snek",
        expected: "Invalid",
    },
    {
        name: prog_defn,
        file: "prog_defn.snek",
        expected: "Invalid",
    },
    {
        name: duplicate_params,
        file: "duplicate_params.snek",
        expected: "",
    },
    {
        name: invalid_let_binding_input,
        file: "invalid_let_binding_input.snek",
        expected: "keyword"
    },
    {
        name: invalid_set_updating_input,
        file: "invalid_set_updating_input.snek",
        expected: "keyword"
    },
    {
        name: break_outside_loop,
        file: "single_loop_double_break.snek",
        expected: "break"
    },
    {
        name: empty_block,
        file: "empty_block.snek",
        expected: "Invalid"
    },
    {
        name: invalid_parse,
        file: "invalid_parse.snek",
        expected: "Invalid",
    },
    {
        name: let_un_bin_unbound,
        file: "let_un_bin_unbound.snek",
        expected: "Unbound variable identifier x",
    },
    {
        name: let_duplicate_binding,
        file: "let_duplicate_binding.snek",
        expected: "Duplicate binding",
    },
    {
        name: let_no_binding,
        file: "let_no_binding.snek",
        expected: "Invalid",
    },
    {
        name: block_parse_fail,
        file: "block_parse_fail.snek",
        expected: "Invalid",
    },
    {
        name: set_unbound,
        file: "set_unbound.snek",
        expected: "Unbound variable identifier y",
    },
    {
        name: number_bounds_fail,
        file: "number_bounds_fail.snek",
        expected: "Invalid",
    },
    {
        name: sexpr_parse_err,
        file: "sexpr_parse_err.snek",
        expected: "Invalid",
    },
    {
        name: invalid_binop,
        file: "invalid_binop.snek",
        expected: "Invalid",
    },
    {
        name: invalid_binop2,
        file: "invalid_binop2.snek",
        expected: "Invalid",
    },
    {
        name: invalid_binop3,
        file: "invalid_binop3.snek",
        expected: "Invalid",
    },
    {
        name: invalid_unop,
        file: "invalid_unop.snek",
        expected: "Invalid",
    },
    {
        name: duplicate_binding,
        file: "duplicate_binding.snek",
        expected: "Duplicate binding",
    },
    {
        name: break_without_loop,
        file: "break_without_loop.snek",
        expected: "break",
    },
    {
        name: invalid_op,
        file: "invalid_op.snek",
        expected: "Invalid",
    }
}