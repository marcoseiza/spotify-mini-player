export const millisecondsToTime = (millis: number) => {
  const seconds = Math.floor((millis / 1000) % 60);
  let secondsStr = seconds.toString();
  if (seconds < 10) secondsStr = `0${seconds}`;
  const minutes = Math.floor((millis / (60 * 1000)) % 60);
  return minutes + ":" + secondsStr;
};
