// @ts-check

/**
 * Double every card in the deck.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} deck with every card doubled
 */
export function seeingDouble(deck) {
      return deck.map(n=>n*2);
}

/**
 *  Creates triplicates of every 3 found in the deck.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} deck with triplicate 3s
 */
export function threeOfEachThree(deck) {
          const c=[];
        for(const i of deck){
           if(i==3) c.push(3,3,3);
           else c.push(i)
        }
    return c;
   
}

/**
 * Extracts the middle two cards from a deck.
 * Assumes a deck is always 10 cards.
 *
 * @param {number[]} deck of 10 cards
 *
 * @returns {number[]} deck with only two middle cards
 */
export function middleTwo(deck) {
        let n=deck.length;
      if(n%2==0){
        return deck.slice((n/2)-1,(n/2)+1);
      }
  else {   
        n=Math.floor(n/2);
        return deck.slice(n,n+1);
  }
      
}

/**
 * Moves the outside two cards to the middle.
 *
 * @param {number[]} deck with even number of cards
 *
 * @returns {number[]} transformed deck
 */

export function sandwichTrick(deck) {
         const f=deck[0];
          const l=deck[deck.length-1];
         const d= deck.slice(1,-1);
          let n=d.length;
            const mid=Math.floor(n/2);
          d.splice(mid, 0,l,f)
        return d;
}

/**
 * Removes every card from the deck except 2s.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} deck with only 2s
 */
export function twoIsSpecial(deck) {
      return deck.filter(n=>n==2);
}

/**
 * Returns a perfectly order deck from lowest to highest.
 *
 * @param {number[]} deck shuffled deck
 *
 * @returns {number[]} ordered deck
 */
export function perfectlyOrdered(deck) {
   return deck.sort((a,b)=>a-b);
}

/**
 * Reorders the deck so that the top card ends up at the bottom.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} reordered deck
 */
export function reorder(deck) {
    let n=deck.length;
     for(let i=0;i<(deck.length/2);i++){
        [deck[i], deck[n - i - 1]] = [deck[n - i - 1], deck[i]];
        
     }
  return deck;
}
