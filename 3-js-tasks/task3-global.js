function first(){
var array1 = Symbol("data")
const obj={
    [array1]:[1,2,3,4,5]
    // [array1] : ["abe","fdgdhsj"]
     
}

var e=obj[array1].shift();

// console.log(e);
// console.log(obj[a]);
return second(e,obj[array1]);

}

function second(e, array){
    var array2=Symbol("data2");
    const obj2={
        [array2]:[11,12,13,14,15]
        // [array2]:["xyzww","dgdhd"]
    } 
    obj2[array2]=[e,...obj2[array2],...array];
    console.log(obj2[array2]);
    console.log(obj2[array2][0]);
    
    return obj2[array2]
}

const promise= new Promise((resolve, reject) => {
         
        const array=first();

        if (typeof array[0] === "number") {
            const sum = array.reduce((a, b) => a + b, 0);
            sum > 35 ? resolve(`Resolved (Number): Sum = ${sum}`) 
                     : reject("Rejected: Sum <= 35");
        }

        
        else if (typeof array[0] === "string") {
            const allValid = array.every(str => str.length > 3);
            allValid ? resolve("Resolved (String): All substrings > 3") 
                     : reject("Rejected: Some strings length <= 3");
        }

        
        else if (typeof array[0] === "boolean") {
            const trueCount = array.filter(v => v === true).length;
            const falseCount = array.filter(v => v === false).length;

            trueCount === falseCount
                ? resolve("Resolved (Boolean): Equal true and false")
                : reject("Rejected: Unequal true and false");
        }

        else {
            reject("Rejected: Unsupported data type");
        }
    });


promise.then(result=>{
    console.log("Promise Resolve",result);
}).catch(error=>{
    console.log("Promise Rejected", error);
});
