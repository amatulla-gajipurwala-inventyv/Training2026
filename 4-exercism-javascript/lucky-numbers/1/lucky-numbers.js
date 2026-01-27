// @ts-check

/**
 * Calculates the sum of the two input arrays.
 *
 * @param {number[]} array1
 * @param {number[]} array2
 * @returns {number} sum of the two arrays
 */
export function twoSum(array1, array2) {
       let n1=0;
       for(let i in array1){
         n1=n1*10+array1[i];
       }
      let n2=0;
      for(let i in array2){
        n2=n2*10+array2[i];
      }
    return n1+n2;

}

/**
 * Checks whether a number is a palindrome.
 *
 * @param {number} value
 * @returns {boolean} whether the number is a palindrome or not
 */
export function luckyNumber(value) {
      let rev=0;
      let val=value;
      while(val!=0){
        rev=rev*10+(val%10);
         val=Math.floor(val/10);
      }
    return rev==value;
}

/**
 * Determines the error message that should be shown to the user
 * for the given input value.
 *
 * @param {string|null|undefined} input
 * @returns {string} error message
 */
export function errorMessage(input) {
      if (input === '' || input === null || input === undefined) {
    return 'Required field';
  }

  if (Number.isNaN(Number(input)) || Number(input) === 0) {
    return 'Must be a number besides 0';
  }

  return '';
}
