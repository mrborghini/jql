# Jan's Query Language

Todo syntax like this:

```bash
table users {
    id int auto;
    age int required;
    name string required;
    email string;
    description string;
}

add_users(10, "man", description="Hello")
```
