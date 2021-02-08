//
// Created by Maxim Orlovsky on 2/2/21.
//

import Foundation

public struct RGB20Asset: Codable {
    public let genesis: String
    public let id: String
    public let ticker: String
    public let name: String
    public let description: String?
    public let fractionalBits: UInt8
    public let date: String
    public let knownCirculating: UInt64
    public let issueLimit: UInt64

    public var fractionalDivider: UInt64 {
        [1, 10, 100,
         1_000, 10_000, 100_000,
         1_000_000, 10_000_000, 100_000_000,
         1_000_000_000, 10_000_000_000, 100_000_000_000,
         1_000_000_000_000, 10_000_000_000_000, 100_000_000_000_000,
         1_000_000_000_000_000, 10_000_000_000_000_000, 100_000_000_000_000_000,
         1_000_000_000_000_000_000, 10_000_000_000_000_000_000][Int(fractionalBits)]
    }

    public var knownCirculatingAssets: Double {
        Double(knownCirculating) / Double(fractionalDivider)
    }
}