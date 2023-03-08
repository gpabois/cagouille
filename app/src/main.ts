import { createApp, h, provide } from 'vue'
import { createPinia } from 'pinia'
import bootstrap from 'bootstrap'
import { DefaultApolloClient, provideApolloClient } from '@vue/apollo-composable'
import VueApolloComponents from '@vue/apollo-components'

import App from './App.vue'
import router from './router'

import './assets/main.css'

import { ApolloClient, createHttpLink, InMemoryCache } from '@apollo/client/core'
import { createApolloProvider } from '@vue/apollo-option'

// HTTP connection to the API
const httpLink = createHttpLink({
  uri: '/graphql',
  credentials: 'include'
})

// Cache implementation
const cache = new InMemoryCache()

// Create the apollo client
const apolloClient = new ApolloClient({
  link: httpLink,
  cache,
  connectToDevTools: true
})

const apolloProvider = createApolloProvider({
  defaultClient: apolloClient,
})

provideApolloClient(apolloClient);

const app = createApp({
    render: () => h(App)
})

app.use(apolloProvider)
app.use(VueApolloComponents)
app.use(createPinia())
app.use(router)

app.mount('#app')
