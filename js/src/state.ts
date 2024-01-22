import { deserialize } from "borsh";
import { Connection, PublicKey } from "@solana/web3.js";
import {
  getHashedNameSync,
  getNameAccountKeySync,
} from "@bonfida/spl-name-service";
import { Buffer } from "buffer";

export const CATEGORY_TLD = new PublicKey(
  "7Eg3kuaLtyGaKBBHiQm4YGGTmyrADrkWiMPzUuUdLua9"
);

export enum Tag {
  Uninitialized,
  CategoryMetadata,
  CategoryMetadataClosed,
  CategoryMember,
  CategoryMemberClosed,
}

export class CategoryMember {
  tag: Tag;
  domainKey: PublicKey;
  name: string;

  static schema = {
    struct: {
      tag: "u8",
      domainKey: { array: { type: "u8", len: 32 } },
      name: "string",
    },
  };

  constructor(obj: { tag: number; name: string; domainKey: Uint8Array }) {
    this.tag = obj.tag as Tag;
    this.name = obj.name;
    this.domainKey = new PublicKey(obj.domainKey);
  }

  static deserialize(data: Buffer): CategoryMember {
    return new CategoryMember(deserialize(this.schema, data) as any);
  }

  static async retrieve(connection: Connection, key: PublicKey) {
    const accountInfo = await connection.getAccountInfo(key);
    if (!accountInfo || !accountInfo.data) {
      throw new Error("State account not found");
    }
    return this.deserialize(accountInfo.data.slice(96));
  }
  static findKey(domainName: string, category: PublicKey) {
    const hashed = getHashedNameSync(domainName);
    const key = getNameAccountKeySync(hashed, undefined, category);
    return key;
  }
}

export class CategoryMetadata {
  tag: Tag;
  nbRegisteredDomain: number;
  name: string;

  static schema = {
    struct: {
      tag: "u8",
      nbRegisteredDomain: "u32",
      name: "string",
    },
  };

  constructor(obj: { tag: number; nbRegisteredDomain: number; name: string }) {
    this.tag = obj.tag as Tag;
    this.nbRegisteredDomain = obj.nbRegisteredDomain;
    this.name = obj.name;
  }

  static deserialize(data: Buffer): CategoryMetadata {
    return new CategoryMetadata(deserialize(this.schema, data) as any);
  }

  static async retrieve(connection: Connection, key: PublicKey) {
    const accountInfo = await connection.getAccountInfo(key);
    if (!accountInfo || !accountInfo.data) {
      throw new Error("State account not found");
    }
    return this.deserialize(accountInfo.data.slice(96));
  }
  static findKey(categoryName: string) {
    const hashed = getHashedNameSync(categoryName);
    const key = getNameAccountKeySync(hashed, undefined, CATEGORY_TLD);
    return key;
  }
}
