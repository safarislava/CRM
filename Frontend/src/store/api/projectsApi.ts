import { baseApi } from './baseApi'
import type { Project, StageWithProjectTitle } from '../../types'

export const projectsApi = baseApi.injectEndpoints({
  endpoints: (builder) => ({
    getDeadlines: builder.query<StageWithProjectTitle[], void>({
      query: () => '/projects/deadlines',
      providesTags: ['Deadline'],
    }),
    getProjects: builder.query<Project[], void>({
      query: () => '/projects',
      providesTags: ['Project'],
    }),
    createProject: builder.mutation<void, { title: string }>({
      query: (body) => ({ url: '/projects', method: 'POST', body }),
      invalidatesTags: ['Project'],
    }),
    deleteProject: builder.mutation<void, string>({
      query: (id) => ({ url: `/projects/${id}`, method: 'DELETE' }),
      invalidatesTags: ['Project', 'Deadline'],
    }),
    renameProject: builder.mutation<void, { id: string; title: string }>({
      query: ({ id, title }) => ({
        url: `/projects/${id}/title`,
        method: 'PATCH',
        body: { title },
      }),
      invalidatesTags: ['Project'],
    }),
  }),
})

export const {
  useGetDeadlinesQuery,
  useGetProjectsQuery,
  useCreateProjectMutation,
  useDeleteProjectMutation,
  useRenameProjectMutation,
} = projectsApi
