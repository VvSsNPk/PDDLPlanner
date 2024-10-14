;Header and description

(define (domain wumpusWorld)

;remove requirements that are not needed
(:requirements :strips :typing :conditional-effects :negative-preconditions :disjunctive-preconditions)

(:types ;todo: enumerate types and their hierarchy here, e.g. car truck bus - vehicle
    isLocation isLocatable - object
    agent consumable obstacle movable - isLocatable
    goal other - isLocation
    key arrow - consumable
    door wumpus - obstacle
    crate trampoline - movable
)

; un-comment following line if constants are needed
;(:constants )

(:predicates ;todo: define predicates here
    (location ?l - isLocatable ?c - isLocation)
    (link ?c - isLocation ?c2 - isLocation)
    (isempty  ?c - isLocation)
    (isLeft ?c - isLocation ?a - isLocation)
    (isRight ?c - isLocation ?a - isLocation)
    (isNorth ?c - isLocation ?a - isLocation)
    (isSouth ?c - isLocation ?a - isLocation)
    (hasConsumable ?k - consumable)
)


(:functions ;todo: define numeric functions here
)

;define actions here
(:action MoveLeft
    :parameters (
        ?c ?a - isLocation
        ?s - agent
        ?t - trampoline
    )
    :precondition (and 
        (isLeft ?c ?a)
        (link ?c ?a)
        (isempty ?a)
        (location ?s ?c)
        (not (location ?t ?c))
    )
    :effect (and 
        (isempty ?c)
        (not (isempty ?a))
        (not (location ?s ?c))
        (location ?s ?a)
    )
)

(:action MoveRight
      :parameters (
        ?c ?a - isLocation
        ?s - agent
        ?t - trampoline
    )
    :precondition (and 
        (isRight ?c ?a)
        (link ?c ?a)
        (isempty ?a)
        (location ?s ?c)
        (not (location ?t ?c))
    )
    :effect (and 
        (isempty ?c)
        (not (isempty ?a))
        (not (location ?s ?c))
        (location ?s ?a)
    )
)

(:action MoveSouth
      :parameters (
        ?c ?a - isLocation
        ?s - agent
        ?t - trampoline  
    )
    :precondition (and 
        (isSouth ?c ?a)
        (link ?c ?a)
        (isempty ?a)
        (location ?s ?c)
        (not (location ?t ?c))
    )
    :effect (and 
        (isempty ?c)
        (not (isempty ?a))
        (not (location ?s ?c))
        (location ?s ?a)
    )
)

(:action MoveNorth 
      :parameters (
        ?c ?a - isLocation
        ?s - agent
        ?t - trampoline  
    )
    :precondition (and 
        (isNorth ?c ?a)
        (link ?c ?a)
        (isempty ?a)
        (location ?s ?c)
        (not (location ?t ?c))
    )
    :effect (and 
        (isempty ?c)
        (not (isempty ?a))
        (not (location ?s ?c))
        (location ?s ?a)
    )
)

(:action UnlockDoorNorth
    :parameters (
        ?s - agent
        ?c ?a - other
        ?d - door
        ?k - key
    )
    :precondition (and 
        (location ?s ?c)
        (location ?d ?a)
        (hasConsumable ?k)
        (link ?c ?a)
        (isNorth ?c ?a)
    )
    :effect (and 
        (not (hasConsumable ?k))
        (not (location ?d ?a))
        (isempty  ?a)
    )
)
(:action UnlockDoorSouth
    :parameters (
        ?s - agent
        ?c ?a - other
        ?d - door
        ?k - key
    )
    :precondition (and
        (location ?s ?c)
        (location ?d ?a)
        (hasConsumable ?k)
        (link ?c ?a)
        (isSouth ?c ?a)
    )
    :effect (and
        (not (hasConsumable ?k))
        (not (location ?d ?a))
        (isempty  ?a)
    )
)
(:action UnlockDoorLeft
    :parameters (
        ?s - agent
        ?c ?a - other
        ?d - door
        ?k - key
    )
    :precondition (and
        (location ?s ?c)
        (location ?d ?a)
        (hasConsumable ?k)
        (isLeft ?c ?a)
        (link ?c ?a)
    )
    :effect (and
        (not (hasConsumable ?k))
        (not (location ?d ?a))
        (isempty  ?a)
    )
)
(:action UnlockDoorRight
    :parameters (
        ?s - agent
        ?c ?a - other
        ?d - door
        ?k - key
    )
    :precondition (and
        (location ?s ?c)
        (location ?d ?a)
        (hasConsumable ?k)
        (isRight ?c ?a)
        (link ?c ?a)
    )
    :effect (and
        (not (hasConsumable ?k))
        (not (location ?d ?a))
        (isempty  ?a)
    )
)
(:action KillWumpusNorth
    :parameters (
        ?s - agent
        ?c ?a - other
        ?w - wumpus
        ?arrow - arrow
    )
    :precondition (and 
        (location ?s ?c)
        (location ?w ?a)
        (link ?c ?a)
        (isNorth ?c ?a)
        (hasConsumable ?arrow)
    )
    :effect (and
        (not (hasConsumable ?arrow))
        (not (location ?w ?a))
        (isempty  ?a)
     )
)
(:action KillWumpusSouth
    :parameters (
        ?s - agent
        ?c ?a - other
        ?w - wumpus
        ?arrow - arrow
    )
    :precondition (and
        (location ?s ?c)
        (location ?w ?a)
        (link ?c ?a)
        (isSouth ?c ?a)
        (hasConsumable ?arrow)
    )
    :effect (and
        (not (hasConsumable ?arrow))
        (not (location ?w ?a))
        (isempty  ?a)
     )
)
(:action KillWumpusLeft
    :parameters (
        ?s - agent
        ?c ?a - other
        ?w - wumpus
        ?arrow - arrow
    )
    :precondition (and
        (location ?s ?c)
        (location ?w ?a)
        (link ?c ?a)
        (isLeft ?c ?a)
        (hasConsumable ?arrow)
    )
    :effect (and
        (not (hasConsumable ?arrow))
        (not (location ?w ?a))
        (isempty  ?a)
     )
)
(:action KillWumpusRight
    :parameters (
        ?s - agent
        ?c ?a - other
        ?w - wumpus
        ?arrow - arrow
    )
    :precondition (and
        (location ?s ?c)
        (location ?w ?a)
        (link ?c ?a)
        (isRight ?c ?a)
        (hasConsumable ?arrow)
    )
    :effect (and
        (not (hasConsumable ?arrow))
        (not (location ?w ?a))
        (isempty  ?a)
     )
)

(:action PushCrateorTrampolineRight
    :parameters (
        ?s - agent
        ?c ?a  - isLocation
        ?an - other
        ?crate - movable
        ?t - trampoline
    )
    :precondition (and
        (link ?c ?a)
        (link ?a ?an)
        (location ?s ?c)
        (location ?crate ?a)
        (isempty  ?an)
        (isRight ?c ?a)
        (isRight ?a ?an)
        (not (location ?t ?c))

    )
    :effect (and 
        (isempty ?c)
        (not (isempty  ?an))
        (location ?crate ?an)
        (not (location ?s ?c))
        (location ?s ?a)
        (not (location ?crate ?a))
    )
)
(:action PushCrateorTrampolineLeft
    :parameters (
        ?s - agent
        ?c ?a  - isLocation
        ?an - other
        ?crate - movable
        ?t - trampoline
    )
    :precondition (and
        (link ?c ?a)
        (link ?a ?an)
        (location ?s ?c)
        (location ?crate ?a)
        (isempty  ?an)
        (isLeft ?c ?a)
        (isLeft ?a ?an)
        (not (location ?t ?c))

    )
    :effect (and
        (isempty ?c)
        (not (isempty  ?an))
        (location ?crate ?an)
        (not (location ?s ?c))
        (location ?s ?a)
        (not (location ?crate ?a))
    )
)
(:action PushCrateorTrampolineSouth
    :parameters (
        ?s - agent
        ?c ?a  - isLocation
        ?an - other
        ?crate - movable
        ?t - trampoline
    )
    :precondition (and
        (link ?c ?a)
        (link ?a ?an)
        (location ?s ?c)
        (location ?crate ?a)
        (isempty  ?an)
        (isSouth ?c ?a)
        (isSouth ?a ?an)
        (not (location ?t ?c))

    )
    :effect (and
        (isempty ?c)
        (not (isempty  ?an))
        (location ?crate ?an)
        (not (location ?s ?c))
        (location ?s ?a)
        (not (location ?crate ?a))
    )
)
(:action PushCrateorTrampolineNorth
    :parameters (
        ?s - agent
        ?c ?a  - isLocation
        ?an - other
        ?crate - movable
        ?t - trampoline
    )
    :precondition (and
        (link ?c ?a)
        (link ?a ?an)
        (location ?s ?c)
        (location ?crate ?a)
        (isempty  ?an)
        (isNorth ?c ?a)
        (isNorth ?a ?an)
        (not (location ?t ?c))

    )
    :effect (and
        (isempty ?c)
        (not (isempty  ?an))
        (location ?crate ?an)
        (not (location ?s ?c))
        (location ?s ?a)
        (not (location ?crate ?a))
    )
)

(:action PickConsumable
    :parameters (
        ?s - agent
        ?c - other
        ?con - consumable
    )
    :precondition (and 
        (location ?s ?c)
        (location ?con ?c)
    )
    :effect (and 
        (not (location ?con ?c))
        (hasConsumable ?con)
    )
)

(:action JumpNorth
    :parameters (
        ?s - agent
        ?t - trampoline
        ?c ?a ?an - other
    )
    :precondition (and
        (location ?s ?c)
        (location ?t ?c)
        (isNorth ?c ?a)
        (isNorth ?a ?an)
        (link ?c ?a)
        (link ?a ?an)
        (isempty  ?an)

    )
    :effect (and
        (not (location ?s ?c))
        (location ?s ?an)
        (isempty  ?c)
        (not (isempty  ?an))
     )
)
(:action JumpSouth
    :parameters (
        ?s - agent
        ?t - trampoline
        ?c ?a ?an - other
    )
    :precondition (and
        (location ?s ?c)
        (location ?t ?c)
        (isSouth ?c ?a)
        (isSouth ?a ?an)
        (link ?c ?a)
        (link ?a ?an)
        (isempty  ?an)

    )
    :effect (and
        (not (location ?s ?c))
        (location ?s ?an)
        (isempty  ?c)
        (not (isempty  ?an))
     )
)
(:action JumpLeft
    :parameters (
        ?s - agent
        ?t - trampoline
        ?c ?a ?an - other
    )
    :precondition (and
        (location ?s ?c)
        (location ?t ?c)
        (isLeft ?c ?a)
        (isLeft ?a ?an)
        (link ?c ?a)
        (link ?a ?an)
        (isempty  ?an)

    )
    :effect (and
        (not (location ?s ?c))
        (location ?s ?an)
        (isempty  ?c)
        (not (isempty  ?an))
     )
)
(:action JumpRight
    :parameters (
        ?s - agent
        ?t - trampoline
        ?c ?a ?an - other
    )
    :precondition (and
        (location ?s ?c)
        (location ?t ?c)
        (isRight ?c ?a)
        (isRight ?a ?an)
        (link ?c ?a)
        (link ?a ?an)
        (isempty  ?an)

    )
    :effect (and
        (not (location ?s ?c))
        (location ?s ?an)
        (isempty  ?c)
        (not (isempty  ?an))
     )
)


(:action JumpGoalNorth
    :parameters (
        ?s - agent
        ?t - trampoline
        ?c - other
        ?g - goal
    )
    :precondition (and
        (isNorth ?c ?g)
        (location ?s ?c)
        (location ?t ?c)
        (link ?c ?g)
    )
    :effect (and
        (not (location ?s ?c))
        (location ?s ?g)
    )
)
(:action JumpGoalSouth
    :parameters (
        ?s - agent
        ?t - trampoline
        ?c - other
        ?g - goal
    )
    :precondition (and
        (isSouth ?c ?g)
        (location ?s ?c)
        (location ?t ?c)
        (link ?c ?g)
    )
    :effect (and
        (not (location ?s ?c))
        (location ?s ?g)
    )
)
(:action JumpGoalLeft
    :parameters (
        ?s - agent
        ?t - trampoline
        ?c - other
        ?g - goal
    )
    :precondition (and
        (isLeft ?c ?g)
        (location ?s ?c)
        (location ?t ?c)
        (link ?c ?g)
    )
    :effect (and
        (not (location ?s ?c))
        (location ?s ?g)
    )
)
(:action JumpGoalRight
    :parameters (
        ?s - agent
        ?t - trampoline
        ?c - other
        ?g - goal
    )
    :precondition (and
        (isRight ?c ?g)
        (location ?s ?c)
        (location ?t ?c)
        (link ?c ?g)
    )
    :effect (and
        (not (location ?s ?c))
        (location ?s ?g)
    )
)
)