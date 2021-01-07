import gql from 'graphql-tag';
import * as Urql from 'urql';
export type Maybe<T> = T | null;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  /** i53: 53-bit signed integer; represented as `i53`/`i64` in Rust, `Float` in GraphQL, `number` in TypeScript. */
  i53: number;
};

export type Mutation = {
  __typename?: 'Mutation';
  addPdf: Pdf;
};


export type MutationAddPdfArgs = {
  content: Scalars['String'];
};

export type Subscription = {
  __typename?: 'Subscription';
  usersSubscription: Pdf;
};

export type Query = {
  __typename?: 'Query';
  listPdfs: Array<Pdf>;
};


export type Pdf = {
  __typename?: 'Pdf';
  id?: Maybe<Scalars['Int']>;
  filesize?: Maybe<Scalars['i53']>;
  name?: Maybe<Scalars['String']>;
  content?: Maybe<Scalars['String']>;
};

export type AddPdfMutationVariables = Exact<{
  content: Scalars['String'];
}>;


export type AddPdfMutation = (
  { __typename?: 'Mutation' }
  & { addPdf: (
    { __typename?: 'Pdf' }
    & Pick<Pdf, 'id' | 'filesize' | 'name' | 'content'>
  ) }
);

export type UsersSubscriptionSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type UsersSubscriptionSubscription = (
  { __typename?: 'Subscription' }
  & { usersSubscription: (
    { __typename?: 'Pdf' }
    & Pick<Pdf, 'id' | 'filesize' | 'name' | 'content'>
  ) }
);

export type ListPdfsQueryVariables = Exact<{ [key: string]: never; }>;


export type ListPdfsQuery = (
  { __typename?: 'Query' }
  & { listPdfs: Array<(
    { __typename?: 'Pdf' }
    & Pick<Pdf, 'id' | 'filesize' | 'name' | 'content'>
  )> }
);


export const AddPdfDocument = gql`
    mutation addPdf($content: String!) {
  addPdf(content: $content) {
    id
    filesize
    name
    content
  }
}
    `;

export function useAddPdfMutation() {
  return Urql.useMutation<AddPdfMutation, AddPdfMutationVariables>(AddPdfDocument);
};
export const UsersSubscriptionDocument = gql`
    subscription usersSubscription {
  usersSubscription {
    id
    filesize
    name
    content
  }
}
    `;

export function useUsersSubscriptionSubscription<TData = UsersSubscriptionSubscription>(options: Omit<Urql.UseSubscriptionArgs<UsersSubscriptionSubscriptionVariables>, 'query'> = {}, handler?: Urql.SubscriptionHandler<UsersSubscriptionSubscription, TData>) {
  return Urql.useSubscription<UsersSubscriptionSubscription, TData, UsersSubscriptionSubscriptionVariables>({ query: UsersSubscriptionDocument, ...options }, handler);
};
export const ListPdfsDocument = gql`
    query listPdfs {
  listPdfs {
    id
    filesize
    name
    content
  }
}
    `;

export function useListPdfsQuery(options: Omit<Urql.UseQueryArgs<ListPdfsQueryVariables>, 'query'> = {}) {
  return Urql.useQuery<ListPdfsQuery>({ query: ListPdfsDocument, ...options });
};