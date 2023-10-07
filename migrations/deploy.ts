import { setProvider }	from '@coral-xyz/anchor';

module.exports = async function (provider: any) {
  setProvider(provider);
};
