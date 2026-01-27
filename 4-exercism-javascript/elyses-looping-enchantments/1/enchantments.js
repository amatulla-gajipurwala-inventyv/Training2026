// @ts-check

/**
 * Determine how many cards of a certain type there are in the deck
 *
 * @param {number[]} stack
 * @param {number} card
 *
 * @returns {number} number of cards of a single type there are in the deck
 */
export function cardTypeCheck(stack, card) {
     let i=0;
        stack.forEach((n)=>{
          if(n===card)i++;
        });
    return i;
}

/**
 * Determine how many cards are odd or even
 *
 * @param {number[]} stack
 * @param {boolean} type the type of value to check for - odd or even
 * @returns {number} number of cards that are either odd or even (depending on `type`)
 */
export function determineOddEvenCards(stack, type) {
     let ans=0;
          if(type){
            for(const i of stack){
              if(i%2==0) ans++;
            }
          }
       else{
           for(const i of stack){
              if(i%2!=0) ans++;
            }
       }
  return ans;
}
