// @ts-nocheck

/*
  real nice pattern for solving and understanding how context works

  the basic idea is:

             ----> subscriber()
            /  
  value ---- ----> subscriber()          whenever the value is changed we basically call all the subscribers' call back that we have stored in the Set()
            \
             ----> subscriber()

*/
function createConfig(config) {
  const subscribers = new Set()

  /*
    when creating a subscriber
      -> config.subscribe()

    subscribers Set = [(value) => {callback}] 
  */
  function subscribe(subscriber) {
    subscribers.add(subscriber)
  }

  /*
    updater is also a callback function passed to the update function
      e.g. config.update((value) => value + 1)  
  */
  function update(updater) {
    updater(config);
    subscribers.forEach((subscriber) => subscriber(config))
  }

  // setting the value
  function set(value) {
    update(value)
    subscribers.forEach((subscriber) => subscriber(config))
  }

  return { subscribe, update, set }
}

export const config = createConfig(JSON.parse(localStorage.getItem("activeConfig")) || {})