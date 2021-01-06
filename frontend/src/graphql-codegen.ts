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
};

export type Subscription = {
  __typename?: 'Subscription';
  usersSubscription: Pdf;
};

export type Query = {
  __typename?: 'Query';
  users: Array<Pdf>;
};


export type QueryUsersArgs = {
  id: Scalars['Int'];
};

export type Pdf = {
  __typename?: 'Pdf';
  id?: Maybe<Scalars['Int']>;
  name?: Maybe<Scalars['String']>;
};

export type UsersSubscriptionSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type UsersSubscriptionSubscription = (
  { __typename?: 'Subscription' }
  & { usersSubscription: (
    { __typename?: 'Pdf' }
    & Pick<Pdf, 'id' | 'name'>
  ) }
);

export type UsersQueryVariables = Exact<{
  id: Scalars['Int'];
}>;


export type UsersQuery = (
  { __typename?: 'Query' }
  & { users: Array<(
    { __typename?: 'Pdf' }
    & Pick<Pdf, 'id' | 'name'>
  )> }
);


export const UsersSubscriptionDocument = gql`
    subscription usersSubscription {
  usersSubscription {
    id
    name
  }
}
    `;

export function useUsersSubscriptionSubscription<TData = UsersSubscriptionSubscription>(options: Omit<Urql.UseSubscriptionArgs<UsersSubscriptionSubscriptionVariables>, 'query'> = {}, handler?: Urql.SubscriptionHandler<UsersSubscriptionSubscription, TData>) {
  return Urql.useSubscription<UsersSubscriptionSubscription, TData, UsersSubscriptionSubscriptionVariables>({ query: UsersSubscriptionDocument, ...options }, handler);
};
export const UsersDocument = gql`
    query users($id: Int!) {
  users(id: $id) {
    id
    name
  }
}
    `;

export function useUsersQuery(options: Omit<Urql.UseQueryArgs<UsersQueryVariables>, 'query'> = {}) {
  return Urql.useQuery<UsersQuery>({ query: UsersDocument, ...options });
};