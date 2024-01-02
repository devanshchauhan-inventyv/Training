let sum;
(()=>{
    array1=[1,2,3,4,5];
    const variable=array1.shift();
   sum= func2(variable,array1);
})();

function func2(element,arr){
    array2=[6,7,8,9];
    array2.unshift(element);
    array2.push(...arr);
    const sum= array2.reduce((acc,current)=>acc+current,0);
    return sum;
      
}

new Promise(function(resolve,reject){
    sum>30?resolve(sum):reject(`sum should be greater than 30`);
})
    .then((sum)=>{
         fetch(`https://jsonplaceholder.typicode.com/photos?_limit=${sum}`)
        .then((response)=>response.json())
        .then((data)=>{;
             for(const iterator of data) {
            console.log(iterator.id+" "+iterator.url);
         }})
         
     })
     .catch((err)=>console.log(err))