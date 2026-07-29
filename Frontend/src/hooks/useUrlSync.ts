import { useEffect, useRef } from 'react'
import { useDispatch, useSelector } from 'react-redux'
import { useLocation, useNavigate } from 'react-router-dom'
import type { AppDispatch, RootState } from '../store'
import { selectProject, selectStage, setUserPageOpen } from '../store/uiSlice'

export function useUrlSync() {
  const dispatch = useDispatch<AppDispatch>()
  const location = useLocation()
  const navigate = useNavigate()

  const selectedProjectId = useSelector((s: RootState) => s.ui.selectedProjectId)
  const selectedStage = useSelector((s: RootState) => s.ui.selectedStage)
  const userPageOpen = useSelector((s: RootState) => s.ui.userPageOpen)

  const isInitialSync = useRef(true)

  // 1. Initial Sync: Parse URL on app load
  useEffect(() => {
    if (!isInitialSync.current) return
    isInitialSync.current = false

    const path = location.pathname
    if (path === '/profile') {
      dispatch(setUserPageOpen(true))
      return
    }

    const matchStage = path.match(/^\/projects\/([^/]+)\/stages\/(\d+)\/(\d+)$/)
    if (matchStage) {
      const [, pId, parentPos, pos] = matchStage
      dispatch(selectProject(pId))
      dispatch(selectStage({ parentPosition: parseInt(parentPos, 10), position: parseInt(pos, 10) }))
      return
    }

    const matchProject = path.match(/^\/projects\/([^/]+)$/)
    if (matchProject) {
      const [, pId] = matchProject
      dispatch(selectProject(pId))
      return
    }
  }, [location.pathname, dispatch])

  // 2. State to URL Sync: Update URL when Redux state changes
  useEffect(() => {
    if (isInitialSync.current) return

    let targetPath = '/'
    if (userPageOpen) {
      targetPath = '/profile'
    } else if (selectedProjectId && selectedStage) {
      targetPath = `/projects/${selectedProjectId}/stages/${selectedStage.parentPosition}/${selectedStage.position}`
    } else if (selectedProjectId) {
      targetPath = `/projects/${selectedProjectId}`
    }

    if (location.pathname !== targetPath) {
      navigate(targetPath, { replace: false })
    }
  }, [selectedProjectId, selectedStage, userPageOpen, location.pathname, navigate])
}
