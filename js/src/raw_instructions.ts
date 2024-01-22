// This file is auto-generated. DO NOT EDIT
import { serialize } from "borsh";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";

export interface AccountKey {
  pubkey: PublicKey;
  isSigner: boolean;
  isWritable: boolean;
}
export class removeMemberInstruction {
  tag: number;
  categoryMember: string;
  static schema = {
    struct: {
      tag: "u8",
      categoryMember: "string",
    },
  };
  constructor(obj: { categoryMember: string }) {
    this.tag = 2;
    this.categoryMember = obj.categoryMember;
  }
  serialize(): Uint8Array {
    return serialize(removeMemberInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    systemProgram: PublicKey,
    nameServiceProgram: PublicKey,
    feePayer: PublicKey,
    categoryMetadata: PublicKey,
    categoryMember: PublicKey,
    centralState: PublicKey,
    signer: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: systemProgram,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: nameServiceProgram,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: feePayer,
      isSigner: true,
      isWritable: true,
    });
    keys.push({
      pubkey: categoryMetadata,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: categoryMember,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: centralState,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: signer,
      isSigner: true,
      isWritable: false,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
export class createCategoryInstruction {
  tag: number;
  categoryName: string;
  static schema = {
    struct: {
      tag: "u8",
      categoryName: "string",
    },
  };
  constructor(obj: { categoryName: string }) {
    this.tag = 0;
    this.categoryName = obj.categoryName;
  }
  serialize(): Uint8Array {
    return serialize(createCategoryInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    systemProgram: PublicKey,
    nameServiceProgram: PublicKey,
    feePayer: PublicKey,
    categoryMetadata: PublicKey,
    centralState: PublicKey,
    categoryTld: PublicKey,
    signer: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: systemProgram,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: nameServiceProgram,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: feePayer,
      isSigner: true,
      isWritable: true,
    });
    keys.push({
      pubkey: categoryMetadata,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: centralState,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: categoryTld,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: signer,
      isSigner: true,
      isWritable: false,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
export class addMemberInstruction {
  tag: number;
  categoryName: string;
  categoryMember: string;
  static schema = {
    struct: {
      tag: "u8",
      categoryName: "string",
      categoryMember: "string",
    },
  };
  constructor(obj: { categoryName: string; categoryMember: string }) {
    this.tag = 1;
    this.categoryName = obj.categoryName;
    this.categoryMember = obj.categoryMember;
  }
  serialize(): Uint8Array {
    return serialize(addMemberInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    systemProgram: PublicKey,
    nameServiceProgram: PublicKey,
    feePayer: PublicKey,
    categoryMetadata: PublicKey,
    categoryMember: PublicKey,
    centralState: PublicKey,
    signer: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: systemProgram,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: nameServiceProgram,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: feePayer,
      isSigner: true,
      isWritable: true,
    });
    keys.push({
      pubkey: categoryMetadata,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: categoryMember,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: centralState,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: signer,
      isSigner: true,
      isWritable: false,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
