import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
  return {
    streamed: {
      message: fetch('http://0.0.0.0:3000/api/hello').then((data) => data.text())
    }
  }
};
