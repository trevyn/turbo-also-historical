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

 export function listPdfsQuery() {
  return query(operationStore(listPdfsDocument));
 }

 export function addPdfMutation() {
  return mutation(operationStore(addPdfDocument));
 }

 export function usersSubscriptionSubscription(handler) {
  return subscription(operationStore(usersSubscriptionDocument), handler);
 }
</script>
