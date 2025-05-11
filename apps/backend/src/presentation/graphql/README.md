# GraphQL API

## GetUsers
```graphql
query {
  users {
    user(id: 1) {
      id
      name
      createdAt
      updatedAt
    }
  }
}
```

## GetUser
```graphql
query {
  users {
    users {
      id
      name
      createdAt
      updatedAt
    }
  }
}
```


## CreateUser
```graphql
mutation CreateUser($input: CreateUserInput!) {
  users {
    createUser(input: $input) {
      id
      name
      createdAt
      updatedAt
    }
  }
}
```

```json
{
  "input": {
    "name": "A"
  }
}
```

## UpdateUser
```graphql
mutation UpdateUser($id: Int!, $input: UpdateUserInput!) {
  users {
    updateUser(id: $id, input: $input) {
      id
      name
      createdAt
      updatedAt
    }
  }
}
````

```json
{
  "id": 1,
  "input": {
    "name": "C"
  }
}
```

## DeleteUser
```graphql
mutation DeleteUser($id: Int!) {
  users {
    deleteUser(id: $id)
  }
}
````

```json
{
  "id": 2
}
```
