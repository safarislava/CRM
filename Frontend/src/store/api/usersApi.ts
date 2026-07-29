import { baseApi } from './baseApi'
import type { Role } from '../../types'

export const usersApi = baseApi.injectEndpoints({
  endpoints: (builder) => ({
    getMe: builder.query<{ username: string; email: string; notifications_enabled: boolean; roles: Role[] }, void>({
      query: () => '/users/me',
      providesTags: ['Me'],
    }),
    updateUsername: builder.mutation<void, { username: string }>({
      query: (body) => ({ url: '/users/me/username', method: 'PATCH', body }),
      invalidatesTags: ['Me'],
    }),
    updatePassword: builder.mutation<void, { current_password: string; new_password: string }>({
      query: (body) => ({ url: '/users/me/password', method: 'PATCH', body }),
    }),
    updateEmail: builder.mutation<void, { email: string }>({
      query: (body) => ({ url: '/users/me/email', method: 'PATCH', body }),
      invalidatesTags: ['Me'],
    }),
    updateNotifications: builder.mutation<void, { enabled: boolean }>({
      query: (body) => ({ url: '/users/me/notifications', method: 'PATCH', body }),
      invalidatesTags: ['Me'],
    }),
    updateRoles: builder.mutation<void, { roles: Role[] }>({
      query: (body) => ({ url: '/users/me/roles', method: 'PATCH', body }),
      invalidatesTags: ['Me'],
    }),
  }),
})

export const {
  useGetMeQuery,
  useUpdateUsernameMutation,
  useUpdatePasswordMutation,
  useUpdateEmailMutation,
  useUpdateNotificationsMutation,
  useUpdateRolesMutation,
} = usersApi
