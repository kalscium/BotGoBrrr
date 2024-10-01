#include "bytecode_stack.h"
#include "bytecode.h"
#include <stdlib.h>

struct bc_stack_node *bc_stack_init(struct bytecode inst)
{
         struct bc_stack_node *node = malloc(sizeof(struct bc_stack_node));

         node->inst = inst;
         node->next = NULL;

         return node;
}

void bc_stack_push(struct bc_stack_node **head_ref, struct bytecode inst)
{
        /* check if the list is empty */
        if (*head_ref == NULL) {
                *head_ref = bc_stack_init(inst);
                return;
        }

        /* allocate and initalise a new node and make it linked to the previous head */
        struct bc_stack_node *new_node = malloc(sizeof(struct bc_stack_node));
        new_node->inst = inst;
        new_node->next = *head_ref;

        /* replace the old head with the new one */
        *head_ref = new_node;
}

struct optional_bytecode bc_stack_pop(struct bc_stack_node **head_ref)
{
        /*
         * check if the list is empty
         *
         * if so, return none
         */
        if (*head_ref == NULL) {
                struct optional_bytecode none;
                none.is_some = false;
                return none;
        }

        /* extract the inst, free the old node, set the head pointer to the next node */
        struct bytecode inst = (*head_ref)->inst;
        struct bc_stack_node *next = (*head_ref)->next;
        free(*head_ref);
        *head_ref = next;

        /* return the inst as some */
        struct optional_bytecode some = {
                true,
                inst,
        };
        return some;
}

void bc_stack_free(struct bc_stack_node **head_ref)
{
        /* iterate through the nodes and free them individually */
        struct bc_stack_node *current = *head_ref;
        while (current != NULL) {
                free(current);
                current = current->next;
        }

        /* set the head reference to NULL */
        *head_ref = NULL;
}
