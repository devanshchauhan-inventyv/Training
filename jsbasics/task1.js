(() => {
    arr1 = [1, 2, 3, 4, 5];
    firstEle = arr1.shift();
    fun2(firstEle, ...arr1);
  })();
  
  function fun2(firstEle, ...arr1) {
    arr2 = [6, 7, 8, 9];
    arr2.unshift(firstEle);
    arr2.push(...arr1);
    const initialValue = 1;
    const sumVal = arr2.reduce(
      (accumulator, currentValue) => accumulator + currentValue,
      initialValue
    );
  
    const myPromise = new Promise((resolve, reject) => {
      sumVal > 30 ? resolve(sumVal) : reject("err");
    });
  
    myPromise
      .then((val) => {
        fetch(`https://picsum.photos/v2/list?limit=${val}`)
          .then((response) => response.json())
          .then((items) => {
            items.map((item) => {
              console.log(item.download_url);
            });
          });
      })
      .catch((err) => alert(err));
  }