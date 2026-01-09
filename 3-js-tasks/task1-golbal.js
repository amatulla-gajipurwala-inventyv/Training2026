

function firstFunction() {
 let array1=[1,2,3,4,5];
//    let array1=["abcde","fdgdhsj"];
    // let array1=[true, false,true];

    const firstElement = array1.shift();
    
    console.log(array1);
    return secondFunction(firstElement, array1);
}

function secondFunction(firstElement, array1) {
    let array2=[11,12,13,14,15];
    // let array2=["xyzww","dgdhd"];   
    // let array2=[true, false,true];
     array2 = [firstElement, ...array2, ...array1];
     console.log(array2);

    return array2;}


  const promise=new Promise((resolve, reject) => {
         
        const array2=firstFunction();
        if (typeof array2[0] === "number") {
            const sum = array2.reduce((a, b) => a + b, 0);
            sum > 35 ? resolve(`Resolved (Number): Sum = ${sum}`) 
                     : reject("Rejected: Sum <= 35");
        }

        
        else if (typeof array2[0] === "string") {
            const allValid = array2.every(str => str.length > 3);
            allValid ? resolve("Resolved (String): All substrings > 3") 
                     : reject("Rejected: Some strings length <= 3");
        }

        
        else if (typeof array2[0] === "boolean") {
            const trueCount = array2.filter(v => v === true).length;
            const falseCount = array2.filter(v => v === false).length;

            trueCount === falseCount
                ? resolve("Resolved (Boolean): Equal true and false")
                : reject("Rejected: Unequal true and false");
        }

        else {
            reject("Rejected: Unsupported data type");
        }
    });
  
promise.then(result=>{
    console.log("Promise resolve", result);
})  .catch(error=>{

console.log("Promise reject", error);
});

