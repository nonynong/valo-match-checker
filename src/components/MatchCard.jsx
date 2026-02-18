import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import './MatchCard.css'

function MatchCard({ match }) {
  const isLive = match.time_until_match === 'LIVE'
  const score1 = parseInt(match.score1) || 0
  const score2 = parseInt(match.score2) || 0
  const isTied = score1 === score2
  const rounds1 = (parseInt(match.team1_round_ct) || 0) + (parseInt(match.team1_round_t) || 0)
  const rounds2 = (parseInt(match.team2_round_ct) || 0) + (parseInt(match.team2_round_t) || 0)
  const hasRoundData = rounds1 > 0 || rounds2 > 0
  const [odds, setOdds] = useState(null)
  const [loadingOdds, setLoadingOdds] = useState(false)

  useEffect(() => {
    const fetchOdds = async () => {
      setLoadingOdds(true)
      try {
        const result = await invoke('get_polymarket_odds', {
          team1: match.team1,
          team2: match.team2
        })
        setOdds(result)
      } catch (err) {
        console.error('Error fetching Polymarket odds:', err)
      } finally {
        setLoadingOdds(false)
      }
    }
    
    fetchOdds()
  }, [match.team1, match.team2])

  const formatOdds = (price) => {
    if (!price) return null
    // Convert price (0-1) to American odds
    if (price >= 0.5) {
      const american = Math.round((price / (1 - price)) * -100)
      return `${american}`
    } else {
      const american = Math.round(((1 - price) / price) * 100)
      return `+${american}`
    }
  }

  const mapLabel = match.map_number
    ? `Map ${match.map_number} • ${(match.current_map || 'Unknown Map').toUpperCase()}`
    : (match.current_map || 'Unknown Map').toUpperCase()

  return (
    <div className={`match-card ${isLive ? 'live' : ''}`}>
      <div className="match-header">
        <div className="match-info">
          <div className="map-name">{mapLabel}</div>
          <div className="match-series">{match.match_series || match.match_event || 'Series'}</div>
        </div>
        {isLive && (
          <div className="live-badge">
            <span className="live-dot"></span>
            LIVE
          </div>
        )}
      </div>

      <div className="match-teams">
        <div className="team">
          <div className="team-row">
            {match.team1_logo ? (
              <img src={match.team1_logo} alt="" className="team-logo" />
            ) : (
              <span className="team-logo team-logo-placeholder" />
            )}
            <span className="team-name">{match.team1}</span>
          </div>
          <div className="team-score" title="Series (maps won)">{score1}</div>
          {hasRoundData && (
            <div className="team-rounds" title="Rounds this map">({rounds1})</div>
          )}
          {odds && (
            <div className="team-odds">
              {loadingOdds ? (
                <span className="odds-loading">...</span>
              ) : odds.team1_odds ? (
                <span className="odds-value">{formatOdds(odds.team1_odds)}</span>
              ) : (
                <span className="odds-na">—</span>
              )}
            </div>
          )}
        </div>
        <div className="team">
          <div className="team-row">
            {match.team2_logo ? (
              <img src={match.team2_logo} alt="" className="team-logo" />
            ) : (
              <span className="team-logo team-logo-placeholder" />
            )}
            <span className="team-name">{match.team2}</span>
          </div>
          <div className="team-score" title="Series (maps won)">{score2}</div>
          {hasRoundData && (
            <div className="team-rounds" title="Rounds this map">({rounds2})</div>
          )}
          {odds && (
            <div className="team-odds">
              {loadingOdds ? (
                <span className="odds-loading">...</span>
              ) : odds.team2_odds ? (
                <span className="odds-value">{formatOdds(odds.team2_odds)}</span>
              ) : (
                <span className="odds-na">—</span>
              )}
            </div>
          )}
        </div>
      </div>


      {odds?.market_url && (
        <a 
          href={odds.market_url} 
          target="_blank" 
          rel="noopener noreferrer"
          className="polymarket-link"
        >
          View on Polymarket →
        </a>
      )}

      {match.match_event && (
        <div className="match-event">{match.match_event}</div>
      )}
    </div>
  )
}

export default MatchCard
