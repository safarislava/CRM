import { baseApi } from './baseApi'
import { setAccessToken, setInitialized, logout } from '../authSlice'

export const authApi = baseApi.injectEndpoints({
  endpoints: (builder) => ({
    register: builder.mutation<void, { username: string; password: string; invite_token: string; email: string }>({
      query: (body) => ({ url: '/users', method: 'POST', body }),
    }),
    createInvite: builder.mutation<{ token: string }, void>({
      query: () => ({ url: '/invites', method: 'POST' }),
    }),
    login: builder.mutation<{ access_token: string }, { username: string; password: string }>({
      query: (body) => ({ url: '/auth/login', method: 'POST', body }),
    }),
    refresh: builder.mutation<{ access_token: string }, void>({
      query: () => ({ url: '/auth/refresh', method: 'POST' }),
      onQueryStarted: async (_arg, { dispatch, queryFulfilled }) => {
        try {
          const { data } = await queryFulfilled
          dispatch(setAccessToken(data.access_token))
        } catch {
          dispatch(setInitialized())
        }
      },
    }),
    logoutApi: builder.mutation<void, void>({
      query: () => ({ url: '/auth/logout', method: 'POST' }),
      onQueryStarted: async (_arg, { dispatch, queryFulfilled }) => {
        await queryFulfilled.catch(() => {})
        dispatch(logout())
      },
    }),
  }),
})

export const {
  useRegisterMutation,
  useCreateInviteMutation,
  useLoginMutation,
  useRefreshMutation,
  useLogoutApiMutation,
} = authApi
