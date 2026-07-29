import { baseApi } from './baseApi'
import type { Attachment, Act } from '../../types'

export const stageFilesApi = baseApi.injectEndpoints({
  endpoints: (builder) => ({
    listAttachments: builder.query<Attachment[], { projectId: string; position: number }>({
      query: ({ projectId, position }) => `/projects/${projectId}/stages/${position}/attachments`,
      providesTags: (_r, _e, { projectId, position }) => [
        { type: 'Attachment' as const, id: `${projectId}-${position}` },
      ],
    }),
    uploadAttachment: builder.mutation<{ id: string }, { projectId: string; position: number; file: File }>({
      query: ({ projectId, position, file }) => {
        const body = new FormData()
        body.append('file', file)
        return { url: `/projects/${projectId}/stages/${position}/attachments`, method: 'POST', body }
      },
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Attachment' as const, id: `${projectId}-${position}` },
        { type: 'Comment' as const, id: `${projectId}-${position}` },
      ],
    }),
    deleteAttachment: builder.mutation<void, { projectId: string; position: number; attachmentId: string }>({
      query: ({ projectId, position, attachmentId }) => ({
        url: `/projects/${projectId}/stages/${position}/attachments/${attachmentId}`,
        method: 'DELETE',
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Attachment' as const, id: `${projectId}-${position}` },
        { type: 'Comment' as const, id: `${projectId}-${position}` },
      ],
    }),
    listActs: builder.query<Act[], { projectId: string; position: number }>({
      query: ({ projectId, position }) => `/projects/${projectId}/stages/${position}/act`,
      providesTags: (_r, _e, { projectId, position }) => [
        { type: 'Act' as const, id: `${projectId}-${position}` },
      ],
    }),
    uploadAct: builder.mutation<void, { projectId: string; position: number; file: File }>({
      query: ({ projectId, position, file }) => {
        const body = new FormData()
        body.append('file', file)
        return { url: `/projects/${projectId}/stages/${position}/act`, method: 'POST', body }
      },
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Act' as const, id: `${projectId}-${position}` },
        { type: 'Comment' as const, id: `${projectId}-${position}` },
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
        { type: 'Stage' as const, id: projectId },
        'Project',
      ],
    }),
    deleteAct: builder.mutation<void, { projectId: string; position: number; actId: string }>({
      query: ({ projectId, position, actId }) => ({
        url: `/projects/${projectId}/stages/${position}/act/${actId}`,
        method: 'DELETE',
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Act' as const, id: `${projectId}-${position}` },
        { type: 'Comment' as const, id: `${projectId}-${position}` },
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
        { type: 'Stage' as const, id: projectId },
        'Project',
      ],
    }),

    // Sub-stage files
    listSubStageActs: builder.query<Act[], { projectId: string; parentPosition: number; position: number }>({
      query: ({ projectId, parentPosition, position }) => `/projects/${projectId}/stages/${parentPosition}/sub/${position}/act`,
      providesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Act' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
      ],
    }),
    uploadSubStageAct: builder.mutation<void, { projectId: string; parentPosition: number; position: number; file: File }>({
      query: ({ projectId, parentPosition, position, file }) => {
        const body = new FormData()
        body.append('file', file)
        return { url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/act`, method: 'POST', body }
      },
      invalidatesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Act' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
        { type: 'Stage' as const, id: `detail-${projectId}-sub-${parentPosition}-${position}` },
        { type: 'Stage' as const, id: projectId },
        'Project',
      ],
    }),
    deleteSubStageAct: builder.mutation<void, { projectId: string; parentPosition: number; position: number; actId: string }>({
      query: ({ projectId, parentPosition, position, actId }) => ({
        url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/act/${actId}`,
        method: 'DELETE',
      }),
      invalidatesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Act' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
        { type: 'Stage' as const, id: `detail-${projectId}-sub-${parentPosition}-${position}` },
        { type: 'Stage' as const, id: projectId },
        'Project',
      ],
    }),
    listSubStageAttachments: builder.query<Attachment[], { projectId: string; parentPosition: number; position: number }>({
      query: ({ projectId, parentPosition, position }) => `/projects/${projectId}/stages/${parentPosition}/sub/${position}/attachments`,
      providesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Attachment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
      ],
    }),
    uploadSubStageAttachment: builder.mutation<void, { projectId: string; parentPosition: number; position: number; file: File }>({
      query: ({ projectId, parentPosition, position, file }) => {
        const body = new FormData()
        body.append('file', file)
        return { url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/attachments`, method: 'POST', body }
      },
      invalidatesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Attachment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
      ],
    }),
    deleteSubStageAttachment: builder.mutation<void, { projectId: string; parentPosition: number; position: number; attachmentId: string }>({
      query: ({ projectId, parentPosition, position, attachmentId }) => ({
        url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/attachments/${attachmentId}`,
        method: 'DELETE',
      }),
      invalidatesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Attachment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
      ],
    }),
  }),
})

export const {
  useListAttachmentsQuery,
  useUploadAttachmentMutation,
  useDeleteAttachmentMutation,
  useListActsQuery,
  useUploadActMutation,
  useDeleteActMutation,
  useListSubStageActsQuery,
  useUploadSubStageActMutation,
  useDeleteSubStageActMutation,
  useListSubStageAttachmentsQuery,
  useUploadSubStageAttachmentMutation,
  useDeleteSubStageAttachmentMutation,
} = stageFilesApi
