/// <reference path="./global.d.ts" />
// @ts-check

/**
 * Implement the functions needed to solve the exercise here.
 * Do not forget to export them so they are available for the
 * tests. Here an example of the syntax as reminder:
 *
 * export function yourFunction(...) {
 *   ...
 * }
 */

export function cookingStatus(timer){
    if (timer === undefined) {
    return 'You forgot to set the timer.';
  }

  
     else if(timer==0)return "Lasagna is done.";
    else return "Not done, please wait.";
}

export function preparationTime(layers,t=2){
  let n=layers.length;
  return n*t;
}
export function quantities(array){
  let s=0;
  let n=0;
  for(let i in array){
    if(array[i]=='noodles')n++;
    else if(array[i]=='sauce')s++;
  }
  n=n*50;
  s=s*.2;
  return {'noodles':n, 'sauce':s,}
}

export function addSecretIngredient(friendsList,myList){
      let n=friendsList.length;
     let t=friendsList[n-1];
     myList.push(t);
  
}
export function scaleRecipe(recipe,p){
    
    let obj={};
   for(let key in recipe){
      obj[key]=(recipe[key]/2)*p;
 
   }
  return obj;
    
}