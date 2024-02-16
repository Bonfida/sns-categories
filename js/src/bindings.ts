import { PublicKey, SystemProgram } from "@solana/web3.js";
import {
  createCategoryInstruction,
  addMemberInstruction,
} from "./raw_instructions";
import { NAME_PROGRAM_ID } from "@bonfida/spl-name-service";
import { CategoryMember, CategoryMetadata, CATEGORY_TLD } from "./state";

/**
 * Mainnet program ID
 */
export const SNS_CATEGORIES_ID = new PublicKey(
  "6c4faDgogWngP5dFsj1AKFRDtn9zbsae3sgZPuaPfgLK"
);

export const SIGNER = new PublicKey(
  "EXYfL8WTxiVAP8P5xQJLB4Y19JkZoP2jKtgnvvBfwAMJ"
);

/**
 * This function can be used as a js binding example.
 * @param feePayer The fee payer of the transaction
 * @param programId The program ID
 * @returns
 */
export const createCategory = async (
  categoryName: string,
  feePayer: PublicKey,
  programId = SNS_CATEGORIES_ID
) => {
  const metadataKey = CategoryMetadata.findKey(categoryName);
  const [cs] = PublicKey.findProgramAddressSync(
    [programId.toBuffer()],
    programId
  );
  const ix = new createCategoryInstruction({ categoryName }).getInstruction(
    programId,
    SystemProgram.programId,
    NAME_PROGRAM_ID,
    feePayer,
    metadataKey,
    cs,
    CATEGORY_TLD,
    SIGNER
  );

  return [ix];
};

export const addMember = async (
  categoryMember: string,
  categoryName: string,
  feePayer: PublicKey,
  programId = SNS_CATEGORIES_ID
) => {
  const metadataKey = CategoryMetadata.findKey(categoryName);
  const memberKey = CategoryMember.findKey(categoryMember, metadataKey);
  const [cs] = PublicKey.findProgramAddressSync(
    [programId.toBuffer()],
    programId
  );
  const ix = new addMemberInstruction({
    categoryMember,
    categoryName,
  }).getInstruction(
    programId,
    SystemProgram.programId,
    NAME_PROGRAM_ID,
    feePayer,
    metadataKey,
    memberKey,
    cs,
    SIGNER
  );

  return [ix];
};
