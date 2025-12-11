# abfs 25' project
all of this code was written in few days without any plans for the future, so its kinda rough. i think rust version should be working, but is't still slow

## Alg

1. spawn `N` agents
2. each agent tries to `find` solution
3. after some time pheromones dissapear
4. each agent `adds` pheromones based on tour length
5. REPEAT `M` TIMES

### Questions?

1. How big should `N` be?
2. How much pheromones should agent leave?
3. How much pheromones we should destroy?
4. How agent calculates its decision?
5. How big should `M` be?

### + Multi threading
