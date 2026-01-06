
    n = 3;
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
               minimum(i+1, j+1),
               minimum(n  - i, n - j)
            ) ;

            row += val + " ";
        }
        console.log(row);
    }

