// @ts-check
//
// The line above enables type checking for this file. Various IDEs interpret
// the @ts-check directive. It will give you helpful autocompletion when
// implementing this exercise.

/**
 * Determines how long it takes to prepare a certain juice.
 *
 * @param {string} name
 * @returns {number} time in minutes
 */
export function timeToMixJuice(name) {
      let t;
     switch(name){
       case 'Pure Strawberry Joy':  t=0.5 ; break;
       case 'Energizer': t=1.5; break;
       case 'Green Garden': t=1.5; break;
         case   'Tropical Island': t=3; break;
            case'All or Nothing': t=5; break;
            default :  t=2.5;
     }
  return t;
}

/**
 * Calculates the number of limes that need to be cut
 * to reach a certain supply.
 *
 * @param {number} wedgesNeeded
 * @param {string[]} limes
 * @returns {number} number of limes cut
 */
export function limesToCut(wedgesNeeded, limes) {
       let n=limes.length;
  let i=0;
  
       while(i<n && wedgesNeeded >0){
            if(limes[i]=='small') wedgesNeeded-=6;
            else if(limes[i]=='medium')wedgesNeeded-=8;
            else if(limes[i]=='large')wedgesNeeded-=10;
           i=i+1;
       }
    return i;
}

/**
 * Determines which juices still need to be prepared after the end of the shift.
 *
 * @param {number} timeLeft
 * @param {string[]} orders
 * @returns {string[]} remaining orders after the time is up
 */
export function remainingOrders(timeLeft, orders) {
      let i=0;
    let n=orders.length;
    let t=timeLeft;
      while(i<n && t>0){
          if(orders[i]=='Pure Strawberry Joy') t-=.5;
          else if(orders[i]=='Energizer')  t-=1.5;
             else if(orders[i]=='Green Garden') t-=1.5;
             else if(orders[i]=='Tropical Island') t-=3;
                else if(orders[i]=='All or Nothing') t-=5;
            else t-=2.5;
           i++;
      }
    let na=[];
      for(;i<n;i++){
        na.push(orders[i]);
      }
  return na;
}
