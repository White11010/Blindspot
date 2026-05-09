import type { KeyMoment } from '../model/gameAnalysis.types';

type TFn = (key: string, ...args: unknown[]) => string;
type TeFn = (key: string) => boolean;

export function getKeyMomentHeadline(moment: KeyMoment, t: TFn, te: TeFn): string {
  const kindKey = `analysis.moveKind.${moment.kind}`;
  const kindLabel = te(kindKey) ? t(kindKey) : moment.kind;
  const key = 'analysis.keyMoment.headline';
  if (te(key)) {
    return t(key, { move: moment.move_number, kind: kindLabel });
  }
  return `${moment.move_number} — ${kindLabel}`;
}

export function getKeyMomentDescription(moment: KeyMoment, t: TFn, te: TeFn): string {
  if (moment.swing_cp < -30) {
    const k = 'analysis.keyMoment.descLostPawns';
    if (te(k)) {
      return t(k, { pawns: ((-moment.swing_cp) / 100).toFixed(1) });
    }
  }
  if (moment.kind === 'brilliant') {
    const k = 'analysis.keyMoment.descBrilliant';
    if (te(k)) {
      return t(k);
    }
  }
  const k = 'analysis.keyMoment.descSwingCp';
  if (te(k)) {
    return t(k, { swing: moment.swing_cp });
  }
  return String(moment.swing_cp);
}
