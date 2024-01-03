let fact=(n)=>{
    if(n==0){
        return 1;
    }
    if(n==1||n==0){
            return 1;
        }
       let ans=n*fact(n-1);
        return ans;
    }

let combination=(n,r)=>{
    if(n<r){
        console.log("Invalid Input ");
    }
    let nfact=fact(n);
    let rfact=fact(r);
    let n_minus_r=fact(n-r);
    let result= nfact/(rfact*n_minus_r);
    console.log(result);

}  
   

(()=>{
    let n=parseInt(prompt("Enter n"));
    let r=parseInt(prompt("Enter r"));
    combination(n,r);

})();