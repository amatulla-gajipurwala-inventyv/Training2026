
    n = 5;
function minimum(a, b){
    if(a<=b){
         return a;
    }
    else return b;

}


    for (let i = 0; i < n; i++) {
        let row = "";

        for (let j = 0; j < n; j++) {
            let val =minimum(
               minimum(i, j),
               minimum(n - 1 - i, n - 1 - j)
            ) + 1;

            row += val + " ";
        }
        console.log(row);
    }

