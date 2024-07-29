import os

current_dir = os.path.dirname(__file__)
test_dir = os.path.join(current_dir, '../lox_test')

with open("others/test.rs", "w") as f:
    for dir in os.listdir(test_dir):
        dir = os.fsdecode(dir)
        dir_path = os.path.join(test_dir, dir)

        test_mod_name = dir
        if test_mod_name in ("while","for","return","super","if"):
            test_mod_name = test_mod_name + "_keyword"

        f.write("mod " + test_mod_name + " {\n")
        f.write("    use super::test;\n")

        for file in os.listdir(dir_path):
            file = os.fsdecode(file)
            file_path = os.path.join(dir_path, file)
            if file.endswith(".lox"):
                test_fn_name = file.replace(".lox", "")
                if test_fn_name in ("if", "else"):
                    test_fn_name = test_fn_name + "_keyword"
                #print(os.path.join(directory, file_path))
                f.write("    #[test]\n")
                f.write("    fn " + test_fn_name + "() {\n")
                f.write("        test(\"./lox_test/" + dir + "/" + file + "\");\n")
                f.write("    }\n")
            else:
                continue
        f.write("}\n")
