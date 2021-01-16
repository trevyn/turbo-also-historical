<script context="module">
 import { operationStore, query, mutation, subscription } from "@urql/svelte";

 const listPdfsDocument = `
  query {
   listPdfs {
    rowid
    name
   }
  }
 `;

 const addPdfDocument = `
  mutation addPdf($content: String!) {
   addPdf(content: $content)
   {
    rowid
    id
    filesize
    name
    content
   }
  }
 `;

 const usersSubscriptionDocument = `
  subscription {
   usersSubscription
   {
    id
   }
  }
 `;

 export function listPdfsQuery(variables, context) {
  query(operationStore(listPdfsDocument, variables, context));
 }

 export function addPdfMutation(variables, context) {
  mutation(operationStore(addPdfDocument, variables, context));
 }

 export function usersSubscriptionSubscription(variables, context, handler) {
  subscription(
   operationStore(usersSubscriptionDocument, variables, context),
   handler
  );
 }

 // const handleSubscription = (messages = [], data) => {
 //  console.log(data);
 //  return [data.usersSubscription, ...messages];
 // };
 // subscription(usersSubscription, handleSubscription);

 // const mutateTodo = mutation(addPdf);
 // function updateTodo(newTitle) {
 //  mutateTodo({ content: newTitle });
 // }
</script>
