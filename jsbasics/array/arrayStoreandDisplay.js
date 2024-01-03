function Store(n,arr){
    if(n==0){
        console.log(arr);
        return;
    }
    num=prompt(`Enter number`);
    arr.push(num);
    Store(n-1,arr);
}
(()=>{
    let n=parseInt(prompt("Enter n"));
    let arr=[];
    Store(n,arr);
    let i=0;


})();
