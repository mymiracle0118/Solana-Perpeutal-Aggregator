import {
  Connection,
  Keypair,
  Signer,
  PublicKey,
  Transaction,
  TransactionInstruction,
  TransactionSignature,
  ConfirmOptions,
  sendAndConfirmRawTransaction,
  sendAndConfirmTransaction,
  RpcResponseAndContext,
  SimulatedTransactionResponse,
  Commitment,
  LAMPORTS_PER_SOL,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  SYSVAR_CLOCK_PUBKEY,
  clusterApiUrl
} from "@solana/web3.js"
import * as bs58 from 'bs58'
import fs from 'fs'
import * as anchor from '@project-serum/anchor'
import { program } from 'commander';
// import { programs } from '@metaplex/js';
import log from 'loglevel';
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";

program.version('0.0.1');
log.setLevel('info');

// const programId = new PublicKey('AirdfxxqajyegRGW1RpY5JfPyYiZ2Z9WYAZxmhKzxoKo')
const presaleId = new PublicKey('presniX9hhdaCKFXD6fkmEs5cNuL6GWmtjAz6u87NMz')
const programId = new PublicKey('zZPBTeoK4ywiLpzpHJxLDLzVe9zybTEw9zgiLxMYqx5')
const pool_address = new PublicKey('BsRHBE5SKCaai8bJzGqmt7Xs8nkj6rNECRim8LUdBzkN')
const idl = JSON.parse(fs.readFileSync('../target/idl/perpeutal_aggregator.json', 'utf8'))
// const { metadata: { Metadata } } = programs

const confirmOption: ConfirmOptions = {
  commitment: 'finalized',
  preflightCommitment: 'finalized',
  skipPreflight: false
}

const sleep = (ms: number) => {
  return new Promise(resolve => setTimeout(resolve, ms));
};

function loadWalletKey(keypair: any): Keypair {
  if (!keypair || keypair == '') {
    throw new Error('Keypair is required!');
  }
  const loaded = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync(keypair).toString())),
  );
  log.info(`wallet public key: ${loaded.publicKey}`);
  log.info(`wallet secret key: ${bs58.encode(loaded.secretKey)}`)
  return loaded;
}

programCommand('set_pause')
  .requiredOption(
    '-k, --keypair <path>',
    'Solana wallet location'
  )
  .action(async (directory, cmd) => {
    try {
      const { env, keypair, info } = cmd.opts()
      const conn = new Connection(clusterApiUrl(env))
      const owner = loadWalletKey(keypair)
      const wallet = new anchor.Wallet(owner)
      // const owner2 = Keypair.fromSecretKey(bs58.decode("eigebi5tVTfHR22FKqC2NgRp3kzmXDdjwJGUkQhGSRTRa7tCvycaftGQcrGJJd5Jyy7TqoRgBkEJ87dtvs6X3Tf"))
      // const wallet2 = new anchor.Wallet(owner2);

      const provider = new anchor.Provider(conn, wallet, confirmOption)
      const program = new anchor.Program(idl, programId, provider)

      let transaction = new Transaction()
      transaction.add(program.instruction.modifyPool(
        true,
        {
          accounts: {
            owner: owner.publicKey,
            pool: pool_address,
            presalePrgram: presaleId
          }
        }
      ))
      const hash = await sendAndConfirmTransaction(conn, transaction, [owner], confirmOption)
      console.log("Transaction ID : " + hash)
    } catch (err) {
      console.log(err)
    }
  })

function programCommand(name: string) {
  return program
    .command(name)
    .option(
      '-e, --env <string>',
      'Solana cluster env name',
      'devnet',
    )
    .option('-l, --log-level <string>', 'log level', setLogLevel);
}

function setLogLevel(value: any, prev: any) {
  if (value === undefined || value === null) {
    return;
  }
  console.log('setting the log value to: ' + value);
  log.setLevel(value);
}

program.parse(process.argv)