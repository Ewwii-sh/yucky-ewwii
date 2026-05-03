# Additional Features

- Built in JSON support within string interpolation **(version 0.7.0+)**

    ```lisp
    ; Example ;
    (defvar my-json-list "[1, 2, 3, 4]")
    (defvar my-json-obj '{ "key1": "value" }')
    
    (button :label "${my-json-list[0]}")
    (button :label "${my-json-obj.key1}")
    ```