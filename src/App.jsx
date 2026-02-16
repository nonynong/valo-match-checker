import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import MatchCard from './components/MatchCard'
import './App.css'

function App() {
  const [matches, setMatches] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState(null)

  const fetchMatches = async () => {
    try {
      setLoading(true)
      setError(null)
      // Call Rust command to fetch matches
      const result = await invoke('get_live_matches')
      setMatches(result || [])
    } catch (err) {
      console.error('Error fetching matches:', err)
      setError(err.message || 'Failed to load matches')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchMatches()
    // Refresh every 30 seconds
    const interval = setInterval(fetchMatches, 30000)
    
    // Close window when clicking outside (on blur)
    const handleBlur = async () => {
      // Small delay to allow clicks inside the window
      setTimeout(async () => {
        try {
          const window = getCurrentWindow()
          const isFocused = await window.isFocused()
          if (!isFocused) {
            await window.hide()
          }
        } catch (err) {
          // Ignore errors
        }
      }, 100)
    }
    
    window.addEventListener('blur', handleBlur)
    
    return () => {
      clearInterval(interval)
      window.removeEventListener('blur', handleBlur)
    }
  }, [])

  if (loading && matches.length === 0) {
    return (
      <div className="app-container">
        <div className="loading-state">
          <div className="loading-spinner"></div>
          <p>Loading live matches...</p>
        </div>
      </div>
    )
  }

  if (error && matches.length === 0) {
    return (
      <div className="app-container">
        <div className="error-state">
          <p>⚠️ {error}</p>
          <button onClick={fetchMatches} className="retry-button">
            Retry
          </button>
        </div>
      </div>
    )
  }

  return (
    <div className="app-container">
      <div className="app-header">
        <div className="header-content">
          <div className="game-pill">
            <span className="game-pill-dot"></span>
            <span>Valorant</span>
          </div>
          <div className="header-text">
            <h1 className="title">Live Match</h1>
          </div>
        </div>
        <button onClick={fetchMatches} className="refresh-button" title="Refresh">
          🔄
        </button>
      </div>

      <div className="matches-list">
        {matches.length === 0 ? (
          <div className="empty-state">
            <p>No live matches at the moment</p>
          </div>
        ) : (
          <MatchCard match={matches[0]} />
        )}
      </div>

      {matches.length > 1 && (
        <div className="app-footer">
          <span className="footer-text">
            {matches.length} live match{matches.length === 1 ? '' : 'es'} • Showing 1
          </span>
        </div>
      )}
    </div>
  )
}

export default App
