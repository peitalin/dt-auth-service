
// Use in browser only

let res = await fetch("https://0.0.0.0:8082/login", {
  method: "POST",
  headers: {
      "Content-Type": "application/json",
  },
  credentials: "same-origin",
  body: JSON.stringify({
      "email": "sirius@hogwarts.com",
      "password": ""
  })
});

let degenAuth = await res.headers.get("degen-auth");
let auth = await res.headers.get("set-cookie");
console.log("auth:", auth, degenAuth);

let data = await res.json();
let jwt = data.jwt;
console.log(data);

///

var res2 = await fetch("https://127.0.0.1:8082/auth/getUser", {
  method: "POST",
  headers: {
      "Content-Type": "application/json",
  },
  credentials: "include",
  body: JSON.stringify({
      "email": "sirius@hogwarts.com",
  })
});
let data = await res2.json();


