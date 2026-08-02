import { baseApi } from './baseApi'
import type { Role } from '../../types'

export interface SystemStatistics {
  total_users: number
  total_projects: number
  total_stages: number
  pending_invitations: number
}

export interface AdminUserItem {
  id: string
  username: string
  email: string
  notifications_enabled: boolean
  created_at: string
  roles: Role[]
}

export interface AdminInvitationItem {
  token: string
  created_by: string
  created_at: string
  expires_at: string
}

export interface LogEntry {
  timestamp?: string
  level: string
  target?: string
  message: string
  raw: Record<string, unknown>
}

export interface SystemLogsResponse {
  files: string[]
  logs: LogEntry[]
}

export const adminApi = baseApi.injectEndpoints({
  endpoints: (builder) => ({
    getAdminStatistics: builder.query<SystemStatistics, void>({
      query: () => '/admin/statistics',
      providesTags: ['AdminStatistics'],
    }),
    getAdminUsers: builder.query<AdminUserItem[], void>({
      query: () => '/admin/users',
      providesTags: ['AdminUsers'],
    }),
    updateUserRoles: builder.mutation<void, { userId: string; roles: Role[] }>({
      query: ({ userId, roles }) => ({
        url: `/admin/users/${userId}/roles`,
        method: 'PATCH',
        body: { roles },
      }),
      invalidatesTags: ['AdminUsers', 'Me'],
    }),
    deleteUser: builder.mutation<void, { userId: string }>({
      query: ({ userId }) => ({
        url: `/admin/users/${userId}`,
        method: 'DELETE',
      }),
      invalidatesTags: ['AdminUsers', 'AdminStatistics'],
    }),
    getAdminInvitations: builder.query<AdminInvitationItem[], void>({
      query: () => '/admin/invitations',
      providesTags: ['AdminInvitations'],
    }),
    createInvitation: builder.mutation<{ token: string }, void>({
      query: () => ({
        url: '/admin/invitations',
        method: 'POST',
      }),
      invalidatesTags: ['AdminInvitations', 'AdminStatistics'],
    }),
    revokeInvitation: builder.mutation<void, { token: string }>({
      query: ({ token }) => ({
        url: `/admin/invitations/${token}`,
        method: 'DELETE',
      }),
      invalidatesTags: ['AdminInvitations', 'AdminStatistics'],
    }),
    getAdminLogs: builder.query<SystemLogsResponse, { level?: string; query?: string; limit?: number }>({
      query: (params) => ({
        url: '/admin/logs',
        params,
      }),
      providesTags: ['AdminLogs'],
    }),
    triggerDigest: builder.mutation<void, void>({
      query: () => ({
        url: '/admin/digest',
        method: 'POST',
      }),
    }),
  }),
})

export const {
  useGetAdminStatisticsQuery,
  useGetAdminUsersQuery,
  useUpdateUserRolesMutation,
  useDeleteUserMutation,
  useGetAdminInvitationsQuery,
  useCreateInvitationMutation,
  useRevokeInvitationMutation,
  useGetAdminLogsQuery,
  useTriggerDigestMutation,
} = adminApi
