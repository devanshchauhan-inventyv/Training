 fetch('https://jsonplaceholder.typicode.com/photos?_limit=10')
.then((response)=>{
    console.log(response);
   return response.json();
})
.then((data)=>{
    console.log(data);
}).catch((error)=>{
    console.log(`E: ${error} `);
})
